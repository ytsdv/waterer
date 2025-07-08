import { invoke } from "@tauri-apps/api/core";
import { getContext, setContext } from "svelte";

export interface Settings {
  sipAmountMl: number;
  notificationIntervalMinutes: number;
  updateCheckIntervalHours: number;
}

class SettingsState {
  settings = $state<Settings>({
    sipAmountMl: 50,
    notificationIntervalMinutes: 10,
    updateCheckIntervalHours: 24,
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
      // const settings = await invoke("get_settings");
      // this.settings = settings;

      // For now, load from localStorage as a fallback
      const savedSettings = localStorage.getItem("waterer-settings");
      if (savedSettings) {
        this.settings = JSON.parse(savedSettings);
        // Ensure new settings have defaults if they don't exist
        if (this.settings.updateCheckIntervalHours === undefined) {
          this.settings.updateCheckIntervalHours = 24;
        }
      }
    } catch (err) {
      this.error = `Failed to load settings: ${err}`;
      console.error("Error loading settings:", err);
    } finally {
      this.loading = false;
    }
  }

  async saveSettings(newSettings: Settings) {
    try {
      this.loading = true;
      this.error = "";

      // TODO: Save settings to backend when implemented
      // await invoke("save_settings", {
      //   sipAmountMl: newSettings.sipAmountMl,
      //   timerIntervalMs: newSettings.notificationIntervalMinutes * 60000
      // });

      // For now, save to localStorage as a fallback
      localStorage.setItem("waterer-settings", JSON.stringify(newSettings));
      this.settings = { ...newSettings };
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

  updateUpdateCheckInterval(hours: number) {
    this.settings.updateCheckIntervalHours = Math.max(
      1,
      Math.min(168, hours) // Max 1 week
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
