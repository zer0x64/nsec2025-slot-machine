// Badge insertion state
export const badgeInserted = $state({
  value: false,
});

// Loading state after card insertion
export const loadingFunds = $state({
  value: false,
});

// Game ending state - track if the game has ended naturally
export const gameEnded = $state({
  value: false,
  reason: null as "nofunds" | "timeout" | "win" | null,
});

// Reset game state
export function resetGameState() {
  gameEnded.value = false;
  gameEnded.reason = null;
  loadingFunds.value = false;
}
