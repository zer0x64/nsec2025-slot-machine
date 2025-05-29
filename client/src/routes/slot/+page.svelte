<script lang="ts">
    import SlotReel from "./slotReel";
    import { getLayoutUrls, getStopIndexes } from "./helper";
    import { getMetadata, getTimerEvents, getCurrentLevel } from "../levelData.svelte";
    import {
        badgeInserted,
        gameEnded,
        loadingFunds,
    } from "../sharedState.svelte";
    import { type SpinResponse } from "../../generated/models";
    import { showError, showFlag, showInfo } from "$lib/modalStore";

    import { onMount } from "svelte";
    import { Tween } from "svelte/motion";
    import { elasticOut } from "svelte/easing";
    import { invoke } from "@tauri-apps/api/core";
    import { ButtonHandler } from "$lib/inputHandler";

    let currentTimer = $state("10:00");
    let timerEvents = getTimerEvents();

    timerEvents.onmessage = (message) => {
        currentTimer = message;

        if (currentTimer == "00:00") {
            // Set game end state for timeout
            gameEnded.value = true;
            gameEnded.reason = "timeout";

            showInfo("Time's up! Your session has ended.");
        }
    };

    let levelMetadata = getMetadata()!;

    let balance = $state(levelMetadata.startingCredits as unknown as number);
    let bet = $state(levelMetadata.denomination);
    let win = new Tween(0, {
        duration: 1000,
        easing: elasticOut, // More bouncy animation
        interpolate: (a, b) => {
            // Default interpolator interpolates floats, we want ints only
            const delta = b - a;
            return (t) => Math.round(a + t * delta);
        },
    });

    let currentStops = $state(Array<number>(levelMetadata.nWheels).fill(1));
    let spinning = $state(false);

    // Simple win message
    let showWinMessage = $state(false);
    let winAmount = $state(0);
    // Track win size for animation variations
    let isBigWin = $state(false);

    // Formatted currency with thousand separators
    const currencyFormatter = new Intl.NumberFormat("en", { 
        style: "currency",
        currency: "USD",
        maximumFractionDigits: 0
    });

    let geometry: [number, number, number];
    let spacing: number;
    if (levelMetadata.nWheels === 3) {
        geometry = [1.25, 1.25, 1];
        spacing = 0.75;
    } else {
        geometry = [1.5, 1.5, 1];
        spacing = 0.1;
    }

    const slotReel = new SlotReel({
        containerElSelector: ".slot-reel",
        symbolUrls: getLayoutUrls(levelMetadata),
        symbolsPerReel: levelMetadata.reelLayout.length,
        initialSegments: Array<number>(levelMetadata.nWheels).fill(1),
        geometryDimensions: geometry,
        radialSegments: 64,
        cylindersCount: levelMetadata.nWheels,
        cylinderSpacingRatio: spacing,
    });

    async function onSpinButtonPressed() {
        // Don't allow spin during win display
        if (showWinMessage) {
            return;
        }

        // If the game has ended, don't allow any more spins
        if (gameEnded.value) {
            return;
        }

        if (spinning) {
            slotReel.quickStop();
        } else {
            await spin();
        }
    }

    async function spin() {
        if (
            spinning ||
            gameEnded.value ||
            loadingFunds.value ||
            showWinMessage
        ) {
            return;
        }

        if (bet > balance || bet <= 0) {
            showError("Invalid bet amount. Please adjust your bet.");
            return;
        }

        spinning = true;
        win.set(0, { duration: 0 });
        balance -= bet;

        const result: SpinResponse = await invoke("spin", { bet });

        // Get a random index in the layout for the symbols on the payline
        currentStops = getStopIndexes(levelMetadata.reelLayout, result.stops);

        slotReel.spinToTarget({
            stopAtSegments: currentStops,
            callback: async () => {
                if (result.payout > 0) {
                    // Determine if this is a big win
                    isBigWin = result.payout > bet * 5;

                    // Show enhanced win message with appropriate style
                    await displayWinMessage(result.payout as unknown as number);
                }

                balance = result.credits as unknown as number;

                if (balance <= 0) {
                    // Set game end state for no funds
                    gameEnded.value = true;
                    gameEnded.reason = "nofunds";

                    showError("No funds left! Game over.");
                }

                if (bet > balance) {
                    bet = balance;
                }

                spinning = false;

                if (balance >= levelMetadata.requiredCredits) {
                    // Set game end state for win
                    gameEnded.value = true;
                    gameEnded.reason = "win";

                    // Show flag modal instead of alert
                    const flag = await invoke("get_flag");
                    showFlag(flag as string);
                }
            },
        });
    }

    // Enhanced win display
    async function displayWinMessage(amount: number) {
        winAmount = amount;
        showWinMessage = true;

        // Start the counter animation from 0 to the win amount
        win.set(0);
        win.target = amount;

        // Determine display duration based on win size
        const displayDuration = isBigWin ? 3000 : 2000;

        // Keep message visible
        await new Promise((resolve) => setTimeout(resolve, displayDuration));

        showWinMessage = false;
    }

    // Create coin elements for the animation, only after message is spawned in
    $effect(() => {
        if (showWinMessage) {
            createCoinElements();
        }
    })

    // Create the coin elements for the animation
    function createCoinElements() {
        const coinContainer = document.querySelector(".win-coin-container");
        if (!coinContainer) {
            console.error("Unable to query .win-coin-container");
            return;
        }

        // Clear any existing coins
        coinContainer.innerHTML = "";

        // Number of coins based on win size
        const numCoins = isBigWin ? 30 : 15;

        for (let i = 0; i < numCoins; i++) {
            const coin = document.createElement("div");
            coin.className = "win-coin";

            // Random position and animation properties
            const size = Math.random() * 20 + 20; // 20-40px
            const left = Math.random() * 100; // 0-100%
            const delay = Math.random() * 0.5; // 0-0.5s delay
            const speed = Math.random() * 1 + 1; // 1-2s duration

            coin.style.cssText = `
                width: ${size}px;
                height: ${size}px;
                left: ${left}%;
                animation-delay: ${delay}s;
                animation-duration: ${speed}s;
            `;

            coinContainer.appendChild(coin);
        }
    }

    // Create button handlers
    let betUpHandler: ButtonHandler;
    let betDownHandler: ButtonHandler;

    function createButtonHandlers() {
        // Handler for increasing bet
        betUpHandler = new ButtonHandler((multiplier: number) => {
            if (
                !showWinMessage &&
                !spinning &&
                !gameEnded.value &&
                !loadingFunds.value
            ) {
                let newBet = bet + levelMetadata.denomination * multiplier;
                if (newBet <= balance) {
                    bet = newBet;
                } else {
                    bet = balance;
                }
            }
        });

        // Handler for decreasing bet
        betDownHandler = new ButtonHandler((multiplier: number) => {
            if (
                !showWinMessage &&
                !spinning &&
                !gameEnded.value &&
                !loadingFunds.value
            ) {
                let newBet = bet - levelMetadata.denomination * multiplier;
                if (newBet >= 1) {
                    bet = newBet;
                } else {
                    bet = levelMetadata.denomination;
                }
            }
        });
    }

    function handleKeydown(event: KeyboardEvent) {
        // Repeated events are handled manually
        if (event.repeat) return;

        // If win message is showing, block all input
        if (showWinMessage) {
            return;
        }

        // If game has ended or funds are loading, ignore all input
        if (gameEnded.value || loadingFunds.value) {
            return;
        }

        if (!badgeInserted.value) {
            return;
        }

        switch (event.code) {
            case "ArrowUp":
                event.preventDefault();
                betUpHandler.buttonDown();
                break;
            case "ArrowDown":
                event.preventDefault();
                betDownHandler.buttonDown();
                break;
            case "Space":
                event.preventDefault();
                onSpinButtonPressed();
                break;
        }
    }

    function handleKeyup(event: KeyboardEvent) {
        switch (event.code) {
            case "ArrowUp":
                event.preventDefault();
                betUpHandler.buttonUp();
                break;
            case "ArrowDown":
                event.preventDefault();
                betDownHandler.buttonUp();
                break;
        }
    }

    onMount(() => {
        createButtonHandlers();
        slotReel.init();
        window.addEventListener("keydown", handleKeydown);
        window.addEventListener("keyup", handleKeyup);
        return () => {
            window.removeEventListener("keydown", handleKeydown);
            window.removeEventListener("keyup", handleKeyup);
            betUpHandler.cleanup();
            betDownHandler.cleanup();
        };
    });
</script>

<!-- Centered layout -->
<div class="flex flex-col h-full w-full max-w-4xl mx-auto">
    <!-- Timer section with better padding -->
    <div
        class="w-full bg-[rgba(0,0,0,0.3)] p-5 text-center border-b border-[rgba(0,204,255,0.2)] rounded-t-lg"
    >
        <span class="text-2xl text-[#00ccff] font-mono tracking-wider">
            <b>⏳ Time left: </b>
            <span class="text-white">{currentTimer}</span>
        </span>
    </div>

    <!-- Game info stats in a row -->
    <div class="w-full bg-[rgba(255,255,255,0.05)] p-6">
        <div class="flex justify-center gap-8 items-center flex-wrap">
            <div
                class="bg-[rgba(0,0,0,0.3)] px-8 py-4 rounded-lg shadow-[0_0_20px_rgba(28,117,188,0.2)] border border-[rgba(0,204,255,0.2)]"
            >
                <span class="text-[#7fdbff] text-lg">Balance</span>
                <br />
                <span class="text-3xl text-[#00ccff] font-bold">{currencyFormatter.format(balance)}</span>
            </div>
            <div
                class="bg-[rgba(0,0,0,0.3)] px-8 py-4 rounded-lg shadow-[0_0_20px_rgba(28,117,188,0.2)] border border-[rgba(0,204,255,0.2)] relative"
            >
                <span class="text-[#7fdbff] text-lg">Win Paid</span>
                <br />
                <span class="text-3xl text-[#00ccff] font-bold win-amount">
                    {currencyFormatter.format(win.current)}
                </span>
            </div>
            <div
                class="bg-[rgba(0,0,0,0.3)] px-8 py-4 rounded-lg shadow-[0_0_20px_rgba(28,117,188,0.2)] border border-[rgba(0,204,255,0.2)]"
            >
                <span class="text-[#7fdbff] text-lg">Current Bet</span>
                <br />
                <span class="text-3xl text-[#00ccff] font-bold">{currencyFormatter.format(bet)}</span>
            </div>
        </div>
    </div>

    <!-- Slot machine frame centered -->
    <div
        class="flex items-center justify-center p-8 bg-[rgba(0,0,0,0.3)] relative"
    >
        <div class="slot-machine-wrap w-full h-[300px]">
            <div class="slot-reel"></div>
            <div class="slot-machine-line"></div>
        </div>
    </div>

    <!-- Controls section -->
    <div class="p-6 text-center bg-[rgba(255,255,255,0.05)] rounded-b-lg">
        <div class="status-indicator mb-3">
            <span class="text-xl text-[#00ccff] font-bold">
                {#if gameEnded.value}
                    GAME OVER
                {:else if showWinMessage}
                    {isBigWin ? "BIG WIN!" : "YOU WIN!"}
                {:else if spinning}
                    REELS SPINNING
                {:else}
                    READY TO SPIN
                {/if}
            </span>
        </div>

        {#if gameEnded.value}
            <div
                class="text-white text-lg mt-4 bg-[rgba(0,0,0,0.3)] p-3 rounded-lg inline-block px-6"
            >
                Remove access card to exit
            </div>
        {/if}
    </div>
</div>

<!-- Enhanced win message overlay -->
{#if showWinMessage}
    <div
        class="fixed inset-0 z-50 flex items-center justify-center overflow-hidden"
        class:big-win-backdrop={isBigWin}
    >
        <!-- Animated background effects -->
        <div class="win-effects">
            <div class="rays"></div>
            <div class="sparkles">
                {#each Array(20) as _}
                    <div class="sparkle"></div>
                {/each}
            </div>
        </div>

        <!-- Win message box -->
        <div class={`win-message-box ${isBigWin ? "big-win" : ""}`}>
            <div class="win-message-title">
                {isBigWin ? "BIG WIN!" : "YOU WIN"}
            </div>
            <div class="win-message-amount">{currencyFormatter.format(winAmount)}</div>
            <div class="win-message-rays"></div>
        </div>

        <!-- Coin animation container -->
        <div class="win-coin-container"></div>
    </div>
{/if}

<style>
    .slot-machine-wrap {
        width: 100%;
        position: relative;
        border-radius: 8px;
        padding: 20px;
        background: rgba(0, 0, 0, 0.5);
        box-shadow: 0 0 30px rgba(0, 204, 255, 0.2);
        border: 1px solid rgba(0, 204, 255, 0.2);
        overflow: hidden;
    }

    .slot-machine-line {
        position: absolute;
        box-sizing: border-box;
        left: -27px;
        right: -27px;
        top: 50%;
        transform: translateY(-50%);
        height: 3px;
        background-color: #00ccff;
        opacity: 0.8;
        box-shadow: 0 0 10px #00ccff;
    }

    .slot-reel {
        height: 100%;
        width: 100%;
    }

    /* Enhanced win message styling */
    .big-win-backdrop {
        background: rgba(0, 0, 0, 0.85);
    }

    .win-effects {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        overflow: hidden;
    }

    .rays {
        position: absolute;
        top: 50%;
        left: 50%;
        width: 200%;
        height: 200%;
        background: conic-gradient(
            from 0deg,
            transparent 0deg,
            rgba(0, 204, 255, 0.1) 10deg,
            transparent 20deg,
            transparent 30deg,
            rgba(0, 204, 255, 0.1) 40deg,
            transparent 50deg,
            transparent 60deg,
            rgba(0, 204, 255, 0.1) 70deg,
            transparent 80deg,
            transparent 90deg,
            rgba(0, 204, 255, 0.1) 100deg,
            transparent 110deg,
            transparent 120deg,
            rgba(0, 204, 255, 0.1) 130deg,
            transparent 140deg,
            transparent 150deg,
            rgba(0, 204, 255, 0.1) 160deg,
            transparent 170deg,
            transparent 180deg,
            rgba(0, 204, 255, 0.1) 190deg,
            transparent 200deg,
            transparent 210deg,
            rgba(0, 204, 255, 0.1) 220deg,
            transparent 230deg,
            transparent 240deg,
            rgba(0, 204, 255, 0.1) 250deg,
            transparent 260deg,
            transparent 270deg,
            rgba(0, 204, 255, 0.1) 280deg,
            transparent 290deg,
            transparent 300deg,
            rgba(0, 204, 255, 0.1) 310deg,
            transparent 320deg,
            transparent 330deg,
            rgba(0, 204, 255, 0.1) 340deg,
            transparent 350deg
        );
        transform: translate(-50%, -50%);
        animation: rotate 60s linear infinite;
    }

    .sparkles {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    }

    .sparkle {
        position: absolute;
        width: 4px;
        height: 4px;
        background-color: white;
        border-radius: 50%;
        opacity: 0;
        animation: twinkle 2s infinite;
    }

    .sparkle:nth-child(odd) {
        background-color: #00ccff;
    }

    .sparkle:nth-child(3n) {
        width: 6px;
        height: 6px;
    }

    .sparkle:nth-child(4n) {
        animation-delay: 0.3s;
    }

    .sparkle:nth-child(5n) {
        animation-delay: 0.5s;
    }

    .sparkle:nth-child(6n) {
        animation-delay: 0.7s;
    }

    .sparkle:nth-child(7n) {
        animation-delay: 0.9s;
    }

    .win-message-box {
        position: relative;
        background-color: rgba(0, 0, 0, 0.7);
        border: 3px solid #00ccff;
        border-radius: 20px;
        padding: 30px 80px;
        text-align: center;
        box-shadow:
            0 0 50px #00ccff,
            0 0 100px rgba(0, 204, 255, 0.5);
        animation: pulse-win 0.5s infinite alternate;
        z-index: 10;
        overflow: hidden;
    }

    .win-message-box.big-win {
        background-color: rgba(0, 10, 30, 0.8);
        border: 6px solid #ffcc00;
        box-shadow:
            0 0 50px #ffcc00,
            0 0 100px rgba(255, 204, 0, 0.5);
        transform: scale(1.2);
        animation: pulse-win-big 0.5s infinite alternate;
    }

    .win-message-rays {
        position: absolute;
        inset: -50%;
        background: radial-gradient(
            circle at center,
            rgba(0, 204, 255, 0.3) 0%,
            transparent 70%
        );
        opacity: 0.5;
        z-index: -1;
    }

    .win-message-box.big-win .win-message-rays {
        background: radial-gradient(
            circle at center,
            rgba(255, 204, 0, 0.3) 0%,
            transparent 70%
        );
        animation: pulse-opacity 1s infinite alternate;
    }

    .win-message-title {
        color: white;
        font-size: 36px;
        font-weight: bold;
        margin-bottom: 10px;
        text-shadow: 0 0 15px rgba(0, 204, 255, 0.8);
        letter-spacing: 2px;
    }

    .win-message-box.big-win .win-message-title {
        color: #ffcc00;
        font-size: 48px;
        text-shadow: 0 0 15px rgba(255, 204, 0, 0.8);
        letter-spacing: 3px;
        animation: text-pulse 0.5s infinite alternate;
    }

    .win-message-amount {
        color: #00ccff;
        font-size: 64px;
        font-weight: bold;
        text-shadow:
            0 0 10px rgba(0, 204, 255, 0.7),
            0 0 20px rgba(0, 204, 255, 0.5);
    }

    .win-message-box.big-win .win-message-amount {
        color: white;
        font-size: 72px;
        text-shadow:
            0 0 10px rgba(255, 255, 255, 0.7),
            0 0 20px rgba(255, 255, 255, 0.5);
        animation: text-glow 0.8s infinite alternate;
    }

    /* Falling coins animation */
    .win-coin-container {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        z-index: 5;
    }

    /* Using global to avoid the class being optimized out */
    :global(.win-coin) {
        position: absolute;
        top: -50px;
        background: radial-gradient(circle at center, #ffde59 30%, #ffbd0a 70%);
        border-radius: 50%;
        box-shadow:
            0 0 5px rgba(255, 215, 0, 0.8),
            0 0 10px rgba(255, 215, 0, 0.4),
            inset 0 0 4px rgba(255, 255, 255, 0.9);
        animation: fall-coin linear forwards;
        z-index: 5;
    }

    :global(.win-coin::before) {
        content: "$";
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        color: rgba(0, 0, 0, 0.6);
        font-weight: bold;
        font-size: 0.6em;
    }

    /* Animations */
    @keyframes pulse-win {
        0% {
            transform: scale(1);
            box-shadow:
                0 0 30px rgba(0, 204, 255, 0.6),
                0 0 60px rgba(0, 204, 255, 0.3);
        }
        100% {
            transform: scale(1.03);
            box-shadow:
                0 0 40px rgba(0, 204, 255, 0.8),
                0 0 80px rgba(0, 204, 255, 0.5);
        }
    }

    @keyframes pulse-win-big {
        0% {
            transform: scale(1.2);
            box-shadow:
                0 0 30px rgba(255, 204, 0, 0.6),
                0 0 60px rgba(255, 204, 0, 0.3);
        }
        100% {
            transform: scale(1.25);
            box-shadow:
                0 0 50px rgba(255, 204, 0, 0.8),
                0 0 100px rgba(255, 204, 0, 0.5);
        }
    }

    @keyframes text-pulse {
        0% {
            transform: scale(1);
        }
        100% {
            transform: scale(1.1);
        }
    }

    @keyframes text-glow {
        0% {
            text-shadow:
                0 0 10px rgba(255, 255, 255, 0.7),
                0 0 20px rgba(255, 255, 255, 0.5);
        }
        100% {
            text-shadow:
                0 0 20px rgba(255, 255, 255, 0.9),
                0 0 30px rgba(255, 255, 255, 0.7),
                0 0 40px rgba(255, 255, 255, 0.5);
        }
    }

    @keyframes fall-coin {
        0% {
            transform: translateY(0) rotate(0deg);
            opacity: 1;
        }
        80% {
            opacity: 1;
        }
        100% {
            transform: translateY(800px) rotate(360deg);
            opacity: 0;
        }
    }

    @keyframes rotate {
        0% {
            transform: translate(-50%, -50%) rotate(0deg);
        }
        100% {
            transform: translate(-50%, -50%) rotate(360deg);
        }
    }

    @keyframes twinkle {
        0%,
        100% {
            opacity: 0;
            transform: scale(0.1);
        }
        50% {
            opacity: 1;
            transform: scale(1);
        }
    }

    @keyframes pulse-opacity {
        0% {
            opacity: 0.3;
        }
        100% {
            opacity: 0.7;
        }
    }

    /* Add a style for the status indicator */
    .status-indicator {
        background: rgba(0, 0, 0, 0.3);
        border-radius: 8px;
        padding: 10px 20px;
        display: inline-block;
        border: 1px solid rgba(0, 204, 255, 0.3);
    }

    /* Slot machine animation */
    :global(body.is-spinning-going) .slot-reel::before {
        z-index: 1;
        content: "";
        position: absolute;
        inset: 0;
        margin: auto;
        backdrop-filter: blur(0);
        animation: blur-animation 3s linear;
    }

    @keyframes blur-animation {
        0% {
            backdrop-filter: blur(0);
            background-color: #00000000;
        }
        20% {
            backdrop-filter: blur(5px);
            background-color: #00000003;
        }
        90% {
            backdrop-filter: blur(5px);
            background-color: #00000003;
        }
        100% {
            backdrop-filter: blur(0);
            background-color: #00000000;
        }
    }
</style>
