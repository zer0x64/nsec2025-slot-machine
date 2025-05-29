import { writable } from "svelte/store";
import { gameEnded } from "../routes/sharedState.svelte";
import { getCurrentLevel } from "../routes/levelData.svelte";

export type ModalType = "success" | "error" | "info" | "flag";

export interface ModalData {
  type: ModalType;
  title: string;
  header: string;
  message: string;
  onClose?: () => void;
  showCloseButton?: boolean;
}

// Initial state - no modal shown
const initialState: ModalData | null = null;

// Create the store
export const modalStore = writable<ModalData | null>(initialState);

// Helper functions
export function showModal(data: ModalData): void {
  // Always show close button with keyboard-only interface
  modalStore.set({
    ...data,
    showCloseButton: true,
  });
}

export function closeModal(): void {
  // If game has ended, don't allow closing the modal through normal means
  if (gameEnded.value) {
    return;
  }

  const unsubscribe = modalStore.subscribe((modal) => {
    if (modal?.onClose) {
      modal.onClose();
    }
  });

  unsubscribe();
  modalStore.set(null);
}

// Force close the modal regardless of game state
export function forceCloseModal(): void {
  const unsubscribe = modalStore.subscribe((modal) => {
    if (modal?.onClose) {
      modal.onClose();
    }
  });

  unsubscribe();
  modalStore.set(null);
}

// Convenience methods for common modals
export function showError(message: string, onClose?: () => void): void {
  // If showing a game-ending error, set appropriate state
  if (message.includes("No funds left")) {
    gameEnded.value = true;
    gameEnded.reason = "nofunds";
  }

  showModal({
    type: "error",
    title: "Error",
    header: "",
    message,
    onClose,
    showCloseButton: true,
  });
}

export function showSuccess(message: string, onClose?: () => void): void {
  showModal({
    type: "success",
    title: "Success",
    header: "",
    message,
    onClose,
    showCloseButton: true,
  });
}

export function showInfo(message: string, onClose?: () => void): void {
  // If showing a time's up message, set appropriate state
  if (message.includes("Time's up")) {
    gameEnded.value = true;
    gameEnded.reason = "timeout";
  }

  showModal({
    type: "info",
    title: "Information",
    header: "",
    message,
    onClose,
    showCloseButton: true,
  });
}

export function showFlag(flag: string, onClose?: () => void): void {
  // Always set the game to ended state with a win when showing a flag
  gameEnded.value = true;
  gameEnded.reason = "win";

  let header: string = "You have reached the maximum balance for this stake level. Please move up to continue playing.";

  if (getCurrentLevel() === 4) {
      header = "You have reached the maximum balance for this machine." +
          "\n⚠️Machine flagged for maintenance review. Please contact support.⚠️";
  }

  showModal({
    type: "flag",
    title: "Congratulations!",
    header: header,
    message: flag,
    onClose,
    showCloseButton: true,
  });
}
