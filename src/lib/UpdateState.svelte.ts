import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getContext, setContext } from "svelte";
import { getSettingsState } from "./SettingsState.svelte";

export interface UpdateInfo {
  available: boolean;
  version?: string;
  body?: string;
  date?: string;
}

export interface UpdateProgress {
  downloaded: number;
  total?: number;
  progress: number;
}

export class UpdateState {
  updateInfo = $state<UpdateInfo>({ available: false });
  isChecking = $state(false);
  isInstalling = $state(false);
  isDownloading = $state(false);
  downloadProgress = $state<UpdateProgress>({ downloaded: 0, progress: 0 });
  error = $state("");
  lastChecked = $state<Date | null>(null);
  updateAvailableNotificationShown = $state(false);
  private periodicCheckInterval: number | null = null;

  constructor() {
    this.setupEventListeners();
    this.startPeriodicChecking();
  }

  private setupEventListeners() {
    $effect(() => {
      const setupListeners = async () => {
        // Listen for update availability from backend
        const unlistenUpdateAvailable = await listen<UpdateInfo>(
          "update-available",
          (event) => {
            this.updateInfo = event.payload;
            this.updateAvailableNotificationShown = false;
          }
        );

        // Listen for download progress
        const unlistenDownloadProgress = await listen<UpdateProgress>(
          "update-download-progress",
          (event) => {
            this.downloadProgress = event.payload;
          }
        );

        // Listen for download started
        const unlistenDownloadStarted = await listen(
          "update-download-started",
          () => {
            this.isDownloading = true;
            this.downloadProgress = { downloaded: 0, progress: 0 };
          }
        );

        // Listen for download finished
        const unlistenDownloadFinished = await listen(
          "update-download-finished",
          () => {
            this.isDownloading = false;
          }
        );

        return () => {
          unlistenUpdateAvailable();
          unlistenDownloadProgress();
          unlistenDownloadStarted();
          unlistenDownloadFinished();
        };
      };

      setupListeners().then((cleanup) => {
        return cleanup;
      });
    });
  }

  private startPeriodicChecking() {
    $effect(() => {
      // Clean up existing interval
      if (this.periodicCheckInterval) {
        clearInterval(this.periodicCheckInterval);
      }

      try {
        const settingsState = getSettingsState();
        const intervalHours = settingsState.settings.updateCheckIntervalHours;
        
        // Set up new interval based on user settings
        const intervalMs = intervalHours * 60 * 60 * 1000; // Convert hours to milliseconds
        
        this.periodicCheckInterval = setInterval(() => {
          this.checkForUpdatesIfDue();
        }, Math.min(intervalMs, 30 * 60 * 1000)); // Check at least every 30 minutes, but respect user setting

      } catch (error) {
        console.warn("Could not set up periodic update checking:", error);
      }

      // Cleanup on component destruction
      return () => {
        if (this.periodicCheckInterval) {
          clearInterval(this.periodicCheckInterval);
        }
      };
    });

    // Initial check after a short delay
    setTimeout(() => {
      this.checkForUpdatesIfDue();
    }, 5000);
  }

  private async checkForUpdatesIfDue() {
    try {
      const settingsState = getSettingsState();
      const intervalHours = settingsState.settings.updateCheckIntervalHours;
      
      if (!this.shouldCheckForUpdates(intervalHours)) {
        return;
      }

      await this.checkForUpdates();
    } catch (error) {
      console.error("Periodic update check failed:", error);
    }
  }

  private shouldCheckForUpdates(intervalHours: number): boolean {
    if (!this.lastChecked) return true;
    
    const now = new Date();
    const timeSinceLastCheck = now.getTime() - this.lastChecked.getTime();
    const intervalMs = intervalHours * 60 * 60 * 1000;
    
    return timeSinceLastCheck >= intervalMs;
  }

  async checkForUpdates(): Promise<boolean> {
    try {
      this.isChecking = true;
      this.error = "";

      const updateInfo = await invoke<UpdateInfo>("check_for_updates");
      this.updateInfo = updateInfo;
      this.lastChecked = new Date();
      
      if (updateInfo.available && !this.updateAvailableNotificationShown) {
        this.updateAvailableNotificationShown = false; // Reset so notification can be shown
      }

      return updateInfo.available;
    } catch (err) {
      this.error = `Failed to check for updates: ${err}`;
      console.error("Update check failed:", err);
      return false;
    } finally {
      this.isChecking = false;
    }
  }

  async installUpdate(): Promise<void> {
    try {
      this.isInstalling = true;
      this.error = "";

      await invoke("install_update");
      // App will restart after installation, so this might not execute
    } catch (err) {
      this.error = `Failed to install update: ${err}`;
      console.error("Update installation failed:", err);
      throw err;
    } finally {
      this.isInstalling = false;
    }
  }

  dismissUpdate() {
    this.updateInfo = { available: false };
    this.updateAvailableNotificationShown = true;
  }

  get hasUpdateAvailable(): boolean {
    return this.updateInfo.available;
  }

  get isUpdateReady(): boolean {
    return this.updateInfo.available && !this.updateAvailableNotificationShown;
  }

  get currentVersion(): string | undefined {
    return this.updateInfo.version;
  }

  get updateDescription(): string | undefined {
    return this.updateInfo.body;
  }

  get formattedLastChecked(): string {
    if (!this.lastChecked) return "Never";
    
    const now = new Date();
    const diffMs = now.getTime() - this.lastChecked.getTime();
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const diffMinutes = Math.floor(diffMs / (1000 * 60));

    if (diffHours >= 24) {
      const diffDays = Math.floor(diffHours / 24);
      return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;
    } else if (diffHours >= 1) {
      return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;
    } else if (diffMinutes >= 1) {
      return `${diffMinutes} minute${diffMinutes > 1 ? 's' : ''} ago`;
    } else {
      return "Just now";
    }
  }
}

const UPDATE_STATE_KEY = Symbol("UPDATE_STATE");

export function setUpdateState() {
  return setContext(UPDATE_STATE_KEY, new UpdateState());
}

export function getUpdateState() {
  return getContext<UpdateState>(UPDATE_STATE_KEY);
}