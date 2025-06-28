use chrono::{self, DateTime, Local, NaiveTime, Timelike, Utc};
use serde::Serialize;
use sqlx::{prelude::FromRow, Pool, Sqlite};

#[derive(Debug, FromRow, Serialize)]
pub struct Sip {
    pub id: i64,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
    pub notified_user: bool,
}

#[derive(Debug)]
pub struct SipState {
    last_sip_timestamp: i64,
    total_sips_today: i64,
    pub total_amount_today: i64,
    total_sips_all_time: i64,
    total_amount_all_time: i64,
    pub last_sip_id: Option<i64>,
    pub notified_user: bool,
}

const SIP_INTERVAL_SECONDS: i64 = 10;

impl SipState {
    pub fn new() -> Self {
        Self {
            last_sip_timestamp: 0,
            total_sips_today: 0,
            total_amount_today: 0,
            total_sips_all_time: 0,
            total_amount_all_time: 0,
            notified_user: false,
            last_sip_id: None,
        }
    }

    pub async fn read_from_db(&self, pool: &Pool<Sqlite>) -> Self {
        let sips = sqlx::query_as::<_, Sip>("SELECT * FROM sips ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .unwrap();

        let last_sip_timestamp_parsed = sips[0].created_at;

        let mut total_amount_all_time = 0;
        let mut total_amount_today = 0;
        let mut total_sips_all_time = 0;
        let mut total_sips_today = 0;

        let local_now = Local::now();
        let start_of_today = local_now
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap();

        for sip in &sips {
            total_amount_all_time += sip.amount;
            total_sips_all_time += 1;

            let sip_local = sip.created_at.with_timezone(&Local);
            if sip_local >= start_of_today {
                total_amount_today += sip.amount;
                total_sips_today += 1;
            }
        }

        let latest_sip = sips.first();

        Self {
            last_sip_timestamp: last_sip_timestamp_parsed.timestamp(),
            total_amount_all_time: total_amount_all_time,
            total_amount_today: total_amount_today,
            total_sips_all_time: total_sips_all_time,
            total_sips_today: total_sips_today,
            notified_user: latest_sip
                .map(|sip| sip.notified_user)
                .unwrap_or_else(|| false),
            last_sip_id: latest_sip.map(|sip| sip.id),
        }
    }

    pub async fn take_sip(&self, amount: i64, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        sqlx::query!("INSERT INTO sips (amount) VALUES (?)", amount)
            .execute(pool)
            .await?;

        let new_state = self.read_from_db(pool).await;

        Ok(new_state)
    }

    pub fn check_if_sip_is_due(&self) -> bool {
        let now = Local::now();
        let last_sip_local = match DateTime::from_timestamp(self.last_sip_timestamp, 0) {
            Some(date_time) => date_time,
            // If the last sip timestamp is not set, return true
            None => return true,
        };
        let diff = now.signed_duration_since(last_sip_local);
        diff.num_seconds() > SIP_INTERVAL_SECONDS
    }

    pub async fn set_notified_user(&mut self, notified_user: bool, pool: &Pool<Sqlite>) {
        if self.last_sip_id.is_none() {
            return;
        }

        let last_sip_id = self.last_sip_id.unwrap();

        let _ = sqlx::query!(
            "UPDATE sips SET notified_user = ? WHERE id = ?",
            notified_user,
            last_sip_id
        )
        .execute(pool)
        .await;

        let new_state = self.read_from_db(pool).await;
        *self = new_state;
    }
}
