import { getContext, setContext } from "svelte";
import type { Sip } from "./components";
import { invoke } from "@tauri-apps/api/core";

class SipState {
  private interval: number | null = null;
  public sips: Sip[] = $state([]);
  public error: string = $state("");
  public loading: boolean = $state(true);

  public totalAmount: number = $derived(
    this.sips.reduce((total, sip) => total + sip.amount, 0)
  );

  constructor() {
    $effect(() => {
      this.updateSips().then(() => {
        this.loading = false;
      });

      this.interval = setInterval(() => {
        this.updateSips();
      }, 1000);

      // Cleanup function
      return () => {
        if (this.interval) {
          clearInterval(this.interval);
          this.interval = null;
        }
      };
    });
  }

  async updateSips() {
    try {
      const result = await invoke<Sip[]>("get_sips");
      this.sips = result;
      this.error = ""; // Clear any previous errors
    } catch (err) {
      this.error = `Failed to load sips: ${err}`;
    }
  }

  async takeSip(amount: number = 50) {
    try {
      console.log("Taking sip of", amount, "ml");
      const result = await invoke<Sip>("take_sip", { amount });
      console.log("Sip taken:", result);

      // Refresh the sips list to get the updated state
      await this.updateSips();

      return result;
    } catch (err) {
      this.error = `Failed to take sip: ${err}`;
      console.error("Error taking sip:", err);
      throw err;
    }
  }

  destroy() {
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
  }
}

const SIP_STATE_KEY = Symbol("SIP_STATE");

export function setSipState() {
  return setContext(SIP_STATE_KEY, new SipState());
}

export function getSipState() {
  return getContext<SipState>(SIP_STATE_KEY);
}
