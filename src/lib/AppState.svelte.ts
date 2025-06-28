import { listen } from "@tauri-apps/api/event";
import { getContext, setContext } from "svelte";

type TAppState = {
  timerStarted: boolean;
};

type UpdateAppStateEventPayload = {
  timer_started: boolean;
};

export class AppState implements TAppState {
  timerStarted: boolean;

  constructor() {
    this.timerStarted = $state(false);

    $effect(() => {
      const unlistenFn = listen<UpdateAppStateEventPayload>(
        "update-app-state",
        (event) => {
          console.log("timer", event.payload.timer_started);
          this.timerStarted = event.payload.timer_started;
        }
      );

      return () => {
        unlistenFn.then((unlisten) => unlisten());
      };
    });
  }
}

const APP_STATE_KEY = Symbol("APP_STATE");

export function setAppState() {
  return setContext(APP_STATE_KEY, new AppState());
}

export function getAppState() {
  return getContext<ReturnType<typeof setAppState>>(APP_STATE_KEY);
}
