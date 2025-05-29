import { Channel, invoke } from "@tauri-apps/api/core";
import { StartResponse } from "../generated/models";

let metadata: StartResponse | null = $state(null);
let timerEvents: Channel<string> = $state(new Channel<string>());
let currentLevel: number = $state(0);

export async function startLevel(level: number) {
  currentLevel = level;
  timerEvents = new Channel<String>();

  metadata = await invoke("start_level", { level, timerEvents });
}

export function getMetadata() {
  return metadata;
}

export function getTimerEvents() {
  return timerEvents;
}

export function getCurrentLevel() {
  return currentLevel;
}
