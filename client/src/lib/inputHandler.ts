// src/lib/inputHandler.ts
export class ButtonHandler {
  private isPressed = false;
  private repeatTimer: number | null = null;
  private initialDelay = 500; // ms before autorepeat starts
  private repeatDelay = 50; // initial repeat delay
  private pressStartTime = 0;
  private accelerationFactor = 2; // how quickly the repeat gets faster
  private accelerationDelay = 1000; // how long until the multiplier kicks in

  private currentMultiplier = 1; // how quickly the repeat gets faster

  constructor(
    private action: (multiplier: number) => void,
    options?: {
      initialDelay?: number;
      repeatDelay?: number;
      accelerationFactor?: number;
      accelerationDelay?: number;
    },
  ) {
    if (options) {
      this.initialDelay = options.initialDelay ?? this.initialDelay;
      this.repeatDelay = options.repeatDelay ?? this.repeatDelay;
      this.accelerationFactor =
        options.accelerationFactor ?? this.accelerationFactor;
      this.accelerationDelay =
        options.accelerationDelay ?? this.accelerationDelay;
    }
  }

  buttonDown() {
    if (this.isPressed) return;

    this.isPressed = true;
    this.pressStartTime = Date.now();

    // Execute immediately on first press
    this.action(this.currentMultiplier);

    // Schedule autorepeat after initial delay
    this.repeatTimer = window.setTimeout(() => {
      this.scheduleNextRepeat();
    }, this.initialDelay);
  }

  buttonUp() {
    this.isPressed = false;

    if (this.repeatTimer !== null) {
      clearTimeout(this.repeatTimer);
      this.currentMultiplier = 1;
      this.repeatTimer = null;
    }
  }

  private scheduleNextRepeat() {
    if (!this.isPressed) return;

    // Execute the action
    this.action(this.currentMultiplier);

    // Calculate how long the button has been held
    const holdTime = Date.now() - this.pressStartTime;

    this.currentMultiplier = Math.pow(
      this.accelerationFactor,
      Math.floor(holdTime / this.accelerationDelay),
    );

    // Schedule next repeat
    this.repeatTimer = window.setTimeout(() => {
      this.scheduleNextRepeat();
    }, this.repeatDelay);
  }

  cleanup() {
    if (this.repeatTimer !== null) {
      clearTimeout(this.repeatTimer);
      this.repeatTimer = null;
      this.currentMultiplier = 1;
    }
  }
}
