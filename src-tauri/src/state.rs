// src-tauri/src/state.rs
use crate::db::DatabaseState;
use std::sync::{Mutex as SyncMutex, MutexGuard, PoisonError};
use tokio::sync::Mutex;

use crate::{AppSettings, AppState, SipState};

/// Wrapper for application settings state
pub struct SettingsState(SyncMutex<AppSettings>);

impl SettingsState {
    pub fn new(settings: AppSettings) -> Self {
        Self(SyncMutex::new(settings))
    }

    pub fn lock(
        &self,
    ) -> Result<MutexGuard<'_, AppSettings>, PoisonError<std::sync::MutexGuard<'_, AppSettings>>>
    {
        self.0.lock()
    }
}

/// Wrapper for sip tracking state
pub struct SipTrackingState(Mutex<SipState>);

impl SipTrackingState {
    pub fn new(sip_state: SipState) -> Self {
        Self(Mutex::new(sip_state))
    }

    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, SipState> {
        self.0.lock().await
    }
}

/// Wrapper for app timer state
pub struct AppTimerState(SyncMutex<AppState>);

impl AppTimerState {
    pub fn new(app_state: AppState) -> Self {
        Self(SyncMutex::new(app_state))
    }

    pub fn lock(
        &self,
    ) -> Result<MutexGuard<'_, AppState>, PoisonError<std::sync::MutexGuard<'_, AppState>>> {
        self.0.lock()
    }
}
