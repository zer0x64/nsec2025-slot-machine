// Taken from https://github.com/ux-ui-pro/slot-reel and modified

import {
  CanvasTexture,
  Clock,
  CylinderGeometry,
  Mesh,
  MeshBasicMaterial,
  OrthographicCamera,
  Scene,
  Texture,
  WebGLRenderer,
} from "three";

type SpinState = {
  stopAtSegments: number[];
  callback: () => void;
};

enum AnimationState {
  Rest,
  Spinning,
  Stopping,
  QuickStopping,
}

interface CylinderState {
  currentSpeed: number;
  targetAngle: number | null;
  status: AnimationState;
}

interface SlotReelConfig {
  onAllSpinsComplete?: () => void;
  stopAtSegments: number[];
  containerElSelector: string;
  symbolUrls: (string | URL)[][];
  geometryDimensions: [number, number, number];
  radialSegments: number;
  symbolsPerReel: number;
  cylinderSpacingRatio: number;
  baseSpinSpeed: number;
  spinAccelFactor: number;
  initialSegments: number[];
  queuedSpinStates: SpinState[];
  cameraDistance?: number;
  cylindersCount: number;
  decelerationEase?: number;
  cylinderStopDelayMs?: number;
}

class SlotReel {
  static readonly defaultOptions: Partial<SlotReelConfig> = {
    containerElSelector: "",
    cameraDistance: 10,
    symbolUrls: [],
    geometryDimensions: [1, 1, 1],
    radialSegments: 16,
    symbolsPerReel: 5,
    cylinderSpacingRatio: 0,
    baseSpinSpeed: 1,
    spinAccelFactor: 30,
    initialSegments: [],
    queuedSpinStates: [],
    cylindersCount: 3,
    onAllSpinsComplete: undefined,
    stopAtSegments: [],
    decelerationEase: 1.5,
    cylinderStopDelayMs: 250,
  };

  private readonly options: SlotReelConfig;
  private scene!: Scene;
  private camera!: OrthographicCamera;
  private renderer!: WebGLRenderer;
  private cylinders: Mesh[] = [];
  private clock: Clock = new Clock();
  private readonly cylinderStates: CylinderState[] = [];
  private currentGlobalState: AnimationState = AnimationState.Rest;
  private currentSpinState: SpinState | null = null;
  private resizeObserver!: ResizeObserver;
  private resizeTimeout?: number;
  private stopTimeout: NodeJS.Timeout | string | number | undefined;

  constructor(options: Partial<SlotReelConfig> = {}) {
    this.options = { ...SlotReel.defaultOptions, ...options } as SlotReelConfig;

    this.cylinderStates = Array.from(
      { length: this.options.cylindersCount },
      () => ({
        currentSpeed: this.options.baseSpinSpeed,
        targetAngle: null,
        status: AnimationState.Rest,
      }),
    );
  }

  async init(): Promise<void> {
    const container = this.validateElement<HTMLDivElement>(
      this.options.containerElSelector,
    );

    if (!container) {
      return;
    }

    this.scene = new Scene();

    const { clientWidth: width, clientHeight: height } = container;

    this.createCamera(width / height, 1, this.options.cameraDistance ?? 10);
    this.createRenderer(width, height, container);

    const textures = await this.stitchSymbolsTextures(this.options.symbolUrls);

    this.createCylinders(textures);
    this.positionCylinders(this.options.cylinderSpacingRatio);

    if (this.options.initialSegments?.length === this.options.cylindersCount) {
      this.setInitialSegments();
    }

    document.body.classList.remove("is-spinning-going", "is-spinning-stopped");

    this.setupResizeObserver(container);

    requestAnimationFrame(this.animate);
  }

  spinToTarget = (target: SpinState): void => {
    if (this.currentGlobalState !== AnimationState.Rest) {
      return;
    }

    document.body.classList.remove("is-spinning-stopped");
    document.body.classList.add("is-spinning-going");

    this.currentSpinState = target;

    this.currentGlobalState = AnimationState.Spinning;

    this.cylinderStates.forEach((state) => {
      state.currentSpeed =
        this.options.baseSpinSpeed * this.options.spinAccelFactor;
      state.status = AnimationState.Spinning;
    });

    const stopAtSegments =
      this.currentSpinState.stopAtSegments.length ===
      this.options.cylindersCount
        ? this.currentSpinState.stopAtSegments
        : this.currentSpinState.stopAtSegments.slice(
            0,
            this.options.cylindersCount,
          );

    this.options.stopAtSegments = stopAtSegments;

    this.stopTimeout = setTimeout(() => {
      this.stopCylinders(
        this.options.cylinderStopDelayMs ?? 250,
        AnimationState.Stopping,
      );
    }, 1000);
  };

  quickStop() {
    if (this.currentGlobalState !== AnimationState.Stopping) {
      clearTimeout(this.stopTimeout);
      this.stopCylinders(10, AnimationState.QuickStopping);
    }
  }

  private validateElement<T extends HTMLElement>(selector: string): T | null {
    return (document.querySelector(selector) as T) ?? null;
  }

  private createCamera(
    aspectRatio: number,
    cameraSize: number,
    cameraDistance: number,
  ): void {
    this.camera = new OrthographicCamera(
      -cameraSize * aspectRatio,
      cameraSize * aspectRatio,
      cameraSize,
      -cameraSize,
      0.1,
      1000,
    );

    this.camera.position.z = cameraDistance;
  }

  private createRenderer(
    width: number,
    height: number,
    container: HTMLElement,
  ): void {
    this.renderer = new WebGLRenderer({
      antialias: true,
      alpha: true,
      premultipliedAlpha: false,
    });
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.setSize(width, height);

    container.appendChild(this.renderer.domElement);
  }

  private async stitchSymbolsTextures(
    layout: (string | URL)[][],
  ): Promise<Texture[]> {
    const loadImage = async (urlLike: string | URL) =>
      new Promise<HTMLImageElement>((resolve, reject) => {
        const url = urlLike instanceof URL ? urlLike.toString() : urlLike;
        const img = new Image();
        img.onload = () => resolve(img);
        img.onerror = reject;
        img.src = url;
      });

    const promises = layout.map(async (symbols) => {
      const symbolImages = await Promise.all(symbols.map(loadImage));

      // Force all images to be the same size
      const width = symbolImages[0].width;
      const height = symbolImages[0].height;

      // Create a canvas for image stitching
      const canvas = document.createElement("canvas");
      canvas.width = width;
      canvas.height = height * symbols.length;

      const ctx = canvas.getContext("2d");
      if (!ctx) {
        throw new Error("Failed to get canvas 2D context");
      }

      // Make sure the background is the desired color
      ctx.fillStyle = "white";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw all the symbols
      let y = 0;
      symbolImages.forEach((img) => {
        ctx.drawImage(img, 0, y, width, height);
        y += height;
      });

      // Create the texture
      const texture = new CanvasTexture(canvas);
      texture.rotation = (3 * Math.PI) / 2;
      texture.center.set(0.5, 0.5);

      return texture;
    });

    return Promise.all(promises);
  }

  private createCylinders(textures: Texture[]): void {
    const {
      geometryDimensions,
      radialSegments,
      symbolsPerReel,
      cylindersCount,
    } = this.options;

    const geometry = new CylinderGeometry(
      ...geometryDimensions,
      radialSegments,
      symbolsPerReel,
      true,
    );

    for (let i = 0; i < cylindersCount; i++) {
      const texture = textures[i % textures.length];
      const material = new MeshBasicMaterial({ map: texture });
      const cylinder = new Mesh(geometry, material);

      cylinder.rotation.z = Math.PI / 2;

      this.scene.add(cylinder);
      this.cylinders.push(cylinder);
    }
  }

  private positionCylinders(cylinderSpacingRatio: number): void {
    const totalWidth = this.camera.right - this.camera.left;
    const count = this.cylinders.length;
    const scale = totalWidth / (count + cylinderSpacingRatio * (count - 1));
    const spacing = scale * cylinderSpacingRatio;

    this.cylinders.forEach((cylinder, index) => {
      cylinder.scale.set(scale, scale, scale);
      cylinder.position.x =
        this.camera.left + scale / 2 + index * (scale + spacing);
    });
  }

  private setInitialSegments(): void {
    const { initialSegments } = this.options;

    this.cylinders.forEach((cylinder, i) => {
      cylinder.rotation.x = this.getSegmentAngle(initialSegments[i]);
    });
  }

  private animate = (): void => {
    const deltaTime = this.clock.getDelta();

    this.cylinders.forEach((cylinder, i) => {
      const state = this.cylinderStates[i];

      switch (state.status) {
        case AnimationState.Spinning:
          cylinder.rotation.x += state.currentSpeed * deltaTime;
          break;

        case AnimationState.Stopping:
        case AnimationState.QuickStopping:
          if (state.targetAngle !== null) {
            const diff = state.targetAngle - cylinder.rotation.x;
            const threshold =
              state.status === AnimationState.QuickStopping ? 0.01 : 0.0025;

            if (Math.abs(diff) < threshold) {
              cylinder.rotation.x = state.targetAngle;
              state.status = AnimationState.Rest;
            } else {
              const smoothness =
                state.status === AnimationState.QuickStopping
                  ? 0.5
                  : (this.options.decelerationEase ?? 1.75);
              const decelerationFactor = Math.min(
                1,
                Math.abs(diff) / (smoothness * 2 * Math.PI),
              );
              const speed = state.currentSpeed * decelerationFactor;

              cylinder.rotation.x += Math.sign(diff) * speed * deltaTime;
            }
          }
          break;

        case AnimationState.Rest:
        default:
          break;
      }
    });

    if (this.currentGlobalState === AnimationState.Stopping) {
      const allRest = this.cylinderStates.every(
        (cs) => cs.status === AnimationState.Rest,
      );

      if (allRest) {
        this.finalizeSpin();
      }
    }

    this.renderer.render(this.scene, this.camera);

    requestAnimationFrame(this.animate);
  };

  private stopCylinders(
    timeout: number,
    stopState: AnimationState.Stopping | AnimationState.QuickStopping,
  ) {
    // The cylinders need different logic for quick stopping and normal stopping, global state doesn't
    this.currentGlobalState = AnimationState.Stopping;

    this.cylinders.forEach((cylinder, i) => {
      setTimeout(() => {
        const state = this.cylinderStates[i];
        const segment = this.options.stopAtSegments[i];
        const targetAngle = this.getSegmentAngle(segment);
        const fullRotations =
          Math.floor(cylinder.rotation.x / (2 * Math.PI)) + 2;

        state.targetAngle = targetAngle + fullRotations * 2 * Math.PI;
        state.status = stopState;
      }, i * timeout);
    });
  }

  private finalizeSpin(): void {
    this.currentGlobalState = AnimationState.Rest;

    document.body.classList.remove("is-spinning-going");
    document.body.classList.add("is-spinning-stopped");

    this.currentSpinState?.callback();
  }

  private getSegmentAngle(segment: number): number {
    const { symbolsPerReel } = this.options;
    const segmentAngle = (2 * Math.PI) / symbolsPerReel;
    const offset = segmentAngle / 2;

    return 2 * Math.PI - ((segment - 1) * segmentAngle + offset);
  }

  private setupResizeObserver(container: HTMLElement): void {
    this.resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        if (entry.target === container) {
          if (this.resizeTimeout) {
            clearTimeout(this.resizeTimeout);
          }

          this.resizeTimeout = window.setTimeout(() => {
            this.onResize(container);
            this.resizeTimeout = undefined;
          }, 150);
        }
      }
    });

    this.resizeObserver.observe(container);
  }

  private onResize(container: HTMLElement): void {
    const { clientWidth: newWidth, clientHeight: newHeight } = container;

    const aspectRatio = newWidth / newHeight;
    const cameraSize = 1;

    this.camera.left = -cameraSize * aspectRatio;
    this.camera.right = cameraSize * aspectRatio;
    this.camera.top = cameraSize;
    this.camera.bottom = -cameraSize;
    this.camera.updateProjectionMatrix();

    this.renderer.setSize(newWidth, newHeight);

    this.positionCylinders(this.options.cylinderSpacingRatio);
  }
}

/* eslint-disable @typescript-eslint/no-namespace, no-redeclare */
namespace SlotReel {
  export type Config = SlotReelConfig;
}
/* eslint-enable @typescript-eslint/no-namespace, no-redeclare */

export default SlotReel;
