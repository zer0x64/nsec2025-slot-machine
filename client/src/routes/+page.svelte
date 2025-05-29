<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import { getMetadata, startLevel } from "./levelData.svelte";
    import { badgeInserted, loadingFunds } from "./sharedState.svelte";
    import { showError } from "$lib/modalStore";
    import { onMount } from "svelte";

    // Stop the previous slot machine
    invoke("stop_level", {});

    // Define a type for our stake objects
    type Stake = {
        name: string;
        color: string;
        textColor: string;
        hoverColor: string;
        sparkle?: boolean;
    };

    let stakes: Stake[] = [
        {
            name: "White",
            color: "#FFFFFF", // White color
            textColor: "#000000", // Black text for contrast
            hoverColor: "#E6E6E6", // Slightly darker white for hover
        },
        {
            name: "Red",
            color: "#FF4136", // Vibrant red
            textColor: "#FFFFFF", // White text for contrast
            hoverColor: "#E03730", // Slightly darker red for hover
        },
        {
            name: "Blue",
            color: "#0074D9", // Vibrant blue
            textColor: "#FFFFFF", // White text for contrast
            hoverColor: "#0065BD", // Slightly darker blue for hover
        },
        {
            name: "Diamond",
            color: "#00CFDD", // Diamond cyan/light blue
            textColor: "#000000", // Black text for better contrast with light color
            hoverColor: "#00B8C4", // Slightly darker cyan for hover
            sparkle: true,
        },
    ];

    let selectedIndex = $state(0);

    async function handleSelect(level: number) {
        try {
            await startLevel(level);
            let metadata = getMetadata();

            if (metadata?.debugInfo) {
                await goto("debug");
            } else {
                await goto("slot");
            }
        } catch (error: unknown) {
            // Safely extract error message
            let errorMessage = "An unknown error occurred";

            if (error instanceof Error) {
                errorMessage = error.message;
            } else if (
                typeof error === "object" &&
                error !== null &&
                "message" in error
            ) {
                errorMessage = String(error.message);
            } else if (typeof error === "string") {
                errorMessage = error;
            }

            showError(`Failed to start level: ${errorMessage}`);
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        // Don't accept key presses during loading
        if (!badgeInserted.value || loadingFunds.value) {
            return;
        }

        switch (event.code) {
            case "ArrowUp":
                event.preventDefault();
                selectedIndex =
                    (selectedIndex - 1 + stakes.length) % stakes.length;
                break;
            case "ArrowDown":
                event.preventDefault();
                selectedIndex = (selectedIndex + 1) % stakes.length;
                break;
            case "Space":
                event.preventDefault();
                handleSelect(selectedIndex + 1);
                break;
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
        return () => {
            window.removeEventListener("keydown", handleKeydown);
        };
    });
</script>

<div class="flex flex-col items-center justify-center h-full">
    <div class="wonderlight-container p-8 max-w-2xl mx-auto">
        <div class="hero-section mb-8">
            <h2
                class="text-[#00ccff] text-center text-3xl font-bold mb-4 shadow-[0_0_10px_rgba(0,204,255,0.5)]"
            >
                SELECT YOUR STAKE LEVEL
            </h2>
            <p class="text-center text-[#7fdbff] italic mb-8 text-xl">
                Choose your adventure in the magical world of WonderLight™
                Slots
            </p>
        </div>

        <div class="stakes-container flex flex-col space-y-4 mb-10">
            {#each stakes as stake, i}
                <div
                    class={`
                        p-5
                        rounded-lg
                        cursor-pointer
                        transition-all duration-300
                        ${stake.sparkle ? "sparkle-container" : ""}
                        ${
                            selectedIndex === i
                                ? "transform scale-105 shadow-[0_0_20px_rgba(28,117,188,0.4)]"
                                : "hover:opacity-90 shadow-[0_0_10px_rgba(28,117,188,0.2)]"
                        }
                        ${selectedIndex === i && stake.sparkle ? "sparkle-active" : ""}
                    `}
                    style={`
                        background-color: ${selectedIndex === i ? stake.color : stake.hoverColor};
                        color: ${stake.textColor};
                        border: 2px solid ${selectedIndex === i ? "#00ccff" : "transparent"};
                        position: relative;
                        overflow: ${stake.sparkle ? "hidden" : "visible"};
                    `}
                >
                    <div
                        class="flex justify-between items-center relative z-10"
                    >
                        <span class="font-bold text-xl">{stake.name} Stake</span
                        >
                        {#if selectedIndex === i}
                            <span class="text-2xl">►</span>
                        {/if}
                    </div>

                    {#if stake.sparkle}
                        <div class="sparkles">
                            {#each Array(12) as _, sparkleIndex}
                                <div
                                    class="sparkle"
                                    style={`--delay: ${sparkleIndex * 0.3}s; --size: ${Math.random() * 10 + 5}px; --top: ${Math.random() * 100}%; --left: ${Math.random() * 100}%;`}
                                ></div>
                            {/each}
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .wonderlight-container {
        color: #fff;
        text-align: center;
    }

    .stakes-container {
        max-width: 500px;
        margin: 0 auto;
    }

    .sparkle-container {
        position: relative;
        overflow: hidden;
    }

    .sparkles {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        z-index: 1;
    }

    .sparkle {
        position: absolute;
        width: var(--size);
        height: var(--size);
        background-color: rgba(255, 255, 255, 0.9);
        border-radius: 50%;
        top: var(--top);
        left: var(--left);
        transform: scale(0);
        animation: sparkle-animation 3s infinite var(--delay);
    }

    .sparkle-active .sparkle {
        animation-play-state: running;
    }

    .sparkle-text {
        position: relative;
    }

    @keyframes sparkle-animation {
        0% {
            transform: scale(0) rotate(0deg);
            opacity: 0;
        }
        20% {
            transform: scale(1) rotate(45deg);
            opacity: 1;
            filter: blur(0px);
            box-shadow:
                0 0 10px 2px white,
                0 0 20px 5px rgba(255, 255, 255, 0.5),
                0 0 30px 10px rgba(0, 207, 221, 0.3);
        }
        50% {
            opacity: 0.8;
        }
        80% {
            transform: scale(0.5) rotate(90deg);
            opacity: 0.3;
        }
        100% {
            transform: scale(0) rotate(135deg);
            opacity: 0;
        }
    }
</style>
