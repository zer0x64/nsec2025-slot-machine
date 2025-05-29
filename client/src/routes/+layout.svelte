<script lang="ts">
    import "../app.css";
    import Modal from "./Modal.svelte";
    import { forceCloseModal } from "$lib/modalStore";
    import {
        badgeInserted,
        gameEnded,
        resetGameState,
        loadingFunds,
    } from "./sharedState.svelte";

    import { goto } from "$app/navigation";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { children } = $props();

    let currentGlobalTime: String = $state("00:00:00");

    $effect(() => {
        // We'll modify this logic slightly to avoid redirect loops
        if (!badgeInserted.value && window.location.pathname !== "/") {
            goto("/");
        }

        // When the badge is removed, reset the game state and return to main menu
        if (!badgeInserted.value && gameEnded.value) {
            resetGameState();
            forceCloseModal(); // Force close any open modal
        }
    });

    const clockEvents = new Channel<String>();
    clockEvents.onmessage = (message) => {
        currentGlobalTime = message;
    };

    invoke("init_clock", {
        clockEvents,
    });

    type GpioEventMessage = {
        eventType: string;
        code: string;
    };

    const gpioEvents = new Channel<GpioEventMessage>();
    gpioEvents.onmessage = (message) => {
        let event = new KeyboardEvent(message.eventType, {
            code: message.code,
        });

        window.dispatchEvent(event);
    };

    invoke("gpio_subscribe", {
        gpioEvents,
    });

    function handleKeydown(event: KeyboardEvent) {
        if (event.code === "KeyB") {
            if (event.shiftKey) {
                if (event.repeat) {
                    return;
                }
                badgeInserted.value = !badgeInserted.value;

                if (badgeInserted.value) {
                    // When card is inserted, start loading funds
                    startFundsLoading();
                } else {
                    // When the badge is removed, reset the game state and force close modals
                    resetGameState();
                    forceCloseModal();
                }
            } else {
                if (event.repeat) {
                    return;
                }
                badgeInserted.value = true;
                // When card is inserted, start loading funds
                startFundsLoading();
            }
        }
    }

    function handleKeyup(event: KeyboardEvent) {
        if (event.code === "KeyB" && !event.shiftKey) {
            badgeInserted.value = false;
            // When the badge is removed, reset the game state and force close modals
            resetGameState();
            forceCloseModal();
        }
    }

    // New function to handle the funds loading animation
    function startFundsLoading() {
        // Start loading animation
        loadingFunds.value = true;

        // After 2 seconds, complete the loading
        setTimeout(() => {
            loadingFunds.value = false;
        }, 2000);
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
        window.addEventListener("keyup", handleKeyup);
        return () => {
            window.removeEventListener("keydown", handleKeydown);
            window.removeEventListener("keyup", handleKeyup);
        };
    });
</script>

<div class="min-h-screen max-h-screen flex flex-col">
    <!-- Header with increased padding -->
    <header
        class="bg-[rgba(255,255,255,0.05)] py-6 border-b border-[rgba(0,204,255,0.2)]"
    >
        <div class="container mx-auto px-4">
            <h1
                class="text-[#00ccff] text-3xl font-bold text-center shadow-[0_0_10px_rgba(0,204,255,0.5)]"
            >
                ✨ WonderLight™ Slots ✨
            </h1>
            <p class="text-center italic text-[#7fdbff] mt-2 text-lg">
                Where Dreams and Fortune Align Under the WonderLight™ Stars
            </p>
        </div>
    </header>

    <!-- Center content both vertically and horizontally -->
    <div class="flex-1 flex flex-col justify-center items-center">
        <input
            type="hidden"
            value="FLAG-b390543ada3ce9332556bed5d047f04487869c58f917828aa47e0df00d72fc76"
        />

        <div class="w-full flex justify-center items-center flex-1">
            <!-- This div ensures the content is centered -->
            <div class="w-full max-w-6xl px-4">
                {@render children()}
            </div>
        </div>
    </div>

    <!-- Footer with increased padding - removed key hint -->
    <footer
        class="bg-[rgba(0,0,0,0.3)] py-4 w-full flex items-center justify-between px-8 border-t border-[rgba(0,204,255,0.2)]"
    >
        <div class="text-[#7fdbff] text-lg">WonderLight™ Gaming Systems</div>
        <div class="text-[#00ccff] text-xl font-mono">
            {currentGlobalTime}
        </div>
    </footer>
</div>

<!-- Access card modal -->
<Modal open={!badgeInserted.value}>
    <div class="text-center p-10">
        <div
            class="text-3xl text-[#00ccff] font-bold mb-6 shadow-[0_0_10px_rgba(0,204,255,0.5)]"
        >
            Access Required
        </div>
        <p class="text-xl text-white mb-8">
            Please insert your access card to play
        </p>
        <div class="sparkle-text text-[#7fdbff] italic">
            ✨ Premium Gaming Awaits ✨
        </div>
    </div>
</Modal>

<!-- Loading funds modal -->
<Modal open={loadingFunds.value}>
    <div class="text-center p-10">
        <div
            class="text-3xl text-[#00ccff] font-bold mb-6 shadow-[0_0_10px_rgba(0,204,255,0.5)]"
        >
            Loading Funds
        </div>
        <p class="text-xl text-white mb-8">
            Please wait while we connect to your bank...
        </p>

        <!-- Loading bar -->
        <div class="loading-bar-container">
            <div class="loading-bar"></div>
        </div>

        <div class="sparkle-text text-[#7fdbff] italic mt-8">
            ✨ Preparing Your Gaming Experience ✨
        </div>
    </div>
</Modal>

<!-- Global modal from the store (for replacing alerts) -->
<Modal />

<style>
    .sparkle-text {
        position: relative;
        letter-spacing: 1px;
    }

    .sparkle-text::before,
    .sparkle-text::after {
        content: "✨";
        display: inline-block;
        animation: sparkle-pulse 2s infinite alternate;
    }

    @keyframes sparkle-pulse {
        0% {
            opacity: 0.5;
            transform: scale(0.8);
        }
        100% {
            opacity: 1;
            transform: scale(1.2);
        }
    }

    /* Loading bar styles */
    .loading-bar-container {
        width: 100%;
        height: 20px;
        background: rgba(0, 0, 0, 0.3);
        border-radius: 10px;
        overflow: hidden;
        border: 1px solid rgba(0, 204, 255, 0.3);
        box-shadow: 0 0 10px rgba(0, 204, 255, 0.2);
    }

    .loading-bar {
        height: 100%;
        width: 0;
        background: linear-gradient(90deg, #00ccff, #33d6ff);
        border-radius: 10px;
        animation: load 2s ease-in-out forwards;
        box-shadow: 0 0 15px rgba(0, 204, 255, 0.5) inset;
    }

    @keyframes load {
        0% {
            width: 0%;
        }
        10% {
            width: 15%;
        }
        30% {
            width: 35%;
        }
        50% {
            width: 45%;
        }
        70% {
            width: 65%;
        }
        85% {
            width: 85%;
        }
        100% {
            width: 100%;
        }
    }
</style>
