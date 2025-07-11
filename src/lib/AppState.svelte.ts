import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getContext, setContext } from "svelte";

type TAppState = {
  timerStarted: boolean;
  updateAppState: () => Promise<void>;
};

type UpdateAppStateEventPayload = {
  timer_started: boolean;
  session_id: number;
  session_start: string;
};

export class AppState implements TAppState {
  timerStarted: boolean = $state(false);
  sessionId: number = $state(0);
  sessionStart: string = $state("");

  constructor() {
    this.updateAppState();

    $effect(() => {
      const unlistenFn = listen<UpdateAppStateEventPayload>(
        "update-app-state",
        (event) => {
          console.log("timer", event.payload.timer_started);
          this.timerStarted = event.payload.timer_started;
          this.sessionId = event.payload.session_id;
          this.sessionStart = event.payload.session_start;
        }
      );

      return () => {
        unlistenFn.then((unlisten) => unlisten());
      };
    });
  }

  async updateAppState(): Promise<void> {
    try {
      const result = await invoke<UpdateAppStateEventPayload>("get_app_state");
      this.timerStarted = result.timer_started;
      this.sessionId = result.session_id;
      this.sessionStart = result.session_start;
    } catch (err) {
      console.error("Failed to load app state:", err);
    }
  }
}

const APP_STATE_KEY = Symbol("APP_STATE");

export function setAppState() {
  return setContext(APP_STATE_KEY, new AppState());
}

export function getAppState() {
  return getContext<ReturnType<typeof setAppState>>(APP_STATE_KEY);
}
