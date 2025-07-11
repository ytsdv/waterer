import { invoke } from "@tauri-apps/api/core";
import { getContext, setContext } from "svelte";

export interface Settings {
  sipAmountMl: number;
  notificationIntervalMinutes: number;
  timerIntervalMs: number;
  notificationsEnabled: boolean;
  startMinimized: boolean;
  dailyGoalMl: number;
}

class SettingsState {
  settings = $state<Settings>({
    sipAmountMl: 50,
    notificationIntervalMinutes: 10,
    timerIntervalMs: 1000,
    notificationsEnabled: true,
    startMinimized: false,
    dailyGoalMl: 2000,
  });
  loading = $state(false);
  error = $state("");

  constructor() {
    this.loadSettings();
  }

  async loadSettings() {
    try {
      this.loading = true;
      this.error = "";

      // TODO: Load settings from backend when implemented
      const settings = await invoke("get_settings");
      this.settings = settings as Settings;

      // For now, load from localStorage as a fallback
      const savedSettings = localStorage.getItem("waterer-settings");
      if (savedSettings) {
        this.settings = JSON.parse(savedSettings);
      }
    } catch (err) {
      this.error = `Failed to load settings: ${err}`;
      console.error("Error loading settings:", err);
    } finally {
      this.loading = false;
    }
  }

  async saveSettings(newSettings: Partial<Settings>) {
    try {
      this.loading = true;
      this.error = "";

      // TODO: Save settings to backend when implemented
      const updatedSettings = await invoke<Settings>("update_settings", {
        settings: {
          sipAmountMl: newSettings.sipAmountMl,
          timerIntervalMs: newSettings.timerIntervalMs,
          notificationsEnabled: newSettings.notificationsEnabled,
          startMinimized: newSettings.startMinimized,
          dailyGoalMl: newSettings.dailyGoalMl,
        },
      });

      console.log("res", updatedSettings);

      // For now, save to localStorage as a fallback
      localStorage.setItem("waterer-settings", JSON.stringify(newSettings));
      this.settings = { ...updatedSettings };
    } catch (err) {
      this.error = `Failed to save settings: ${err}`;
      console.error("Error saving settings:", err);
      throw err;
    } finally {
      this.loading = false;
    }
  }

  updateSipAmount(amount: number) {
    this.settings.sipAmountMl = Math.max(1, Math.min(500, amount));
  }

  updateNotificationInterval(minutes: number) {
    this.settings.notificationIntervalMinutes = Math.max(
      1,
      Math.min(180, minutes)
    );
  }
}

const SETTINGS_STATE_KEY = Symbol("SETTINGS_STATE");

export function setSettingsState() {
  return setContext(SETTINGS_STATE_KEY, new SettingsState());
}

export function getSettingsState() {
  return getContext<SettingsState>(SETTINGS_STATE_KEY);
}
