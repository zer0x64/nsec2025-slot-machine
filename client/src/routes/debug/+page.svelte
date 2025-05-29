<script lang="ts">
    import { goto } from "$app/navigation";
    import { getMetadata, getTimerEvents } from "../levelData.svelte";
    import { chunkString, hex } from "$lib/utils";
    import { onMount } from "svelte";
    import { showInfo } from "$lib/modalStore";
    import { gameEnded, loadingFunds } from "../sharedState.svelte";

    let metadata = getMetadata();
    let timerEvents = getTimerEvents();

    let currentTimer = $state("10:00");

    timerEvents.onmessage = (message) => {
        currentTimer = message;

        if (currentTimer == "00:00") {
            // Set game end state for timeout
            gameEnded.value = true;
            gameEnded.reason = "timeout";

            showInfo("Time's up! Your session has ended.");
        }
    };

    const charsPerPage = 63 * 12; // With current css, there's 63 chars per line
    let debugInfo = chunkString(hex(metadata!.debugInfo!), charsPerPage);
    let currentPage = $state(0);

    function handleKeydown(event: KeyboardEvent) {
        // If game has ended or funds are loading, ignore all input
        if (gameEnded.value || loadingFunds.value) {
            return;
        }

        switch (event.code) {
            case "ArrowUp":
                event.preventDefault();
                currentPage = Math.max(currentPage - 1, 0);
                break;
            case "ArrowDown":
                event.preventDefault();
                currentPage = Math.min(currentPage + 1, debugInfo.length - 1);
                break;
            case "Space":
                event.preventDefault();
                goto("slot");
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

<!-- Completely centered layout -->
<div class="flex items-center justify-center h-full">
    <div class="wonderlight-card w-full max-w-[975px] mx-auto p-8">
        <h2 class="wonderlight-heading text-3xl text-center mb-6">
            ⛔️ ERROR ⛔️
        </h2>

        <div class="timer-display mb-6 text-center">
            <div class="text-[#7fdbff] mb-2 text-xl">Time Remaining:</div>
            <div class="text-white text-2xl font-mono tracking-wider">
                <b>{currentTimer}</b>
            </div>
        </div>

        <div
            class="debug-container bg-[rgba(0,0,0,0.3)] p-6 rounded-lg border border-[rgba(0,204,255,0.2)]"
        >
            <h3 class="text-[#7fdbff] mb-2 text-xl">Debug Data:</h3>
            <div class="flex flex-row gap-2 bg-white p-5 rounded-sm w-full mx-auto">
                <code
                    class="font-mono text-[20px] text-black block break-all leading-tight tracking-wide"
                >
                    {debugInfo[currentPage]}
                </code>

                <!-- Pagination -->
                <div class="flex flex-col items-center justify-center gap-4">
                    <svg width=16 height=16>
                        <polygon points="0,16 8,0 16,16" fill="black" />
                    </svg>
                    <div class="flex flex-col items-center gap-2">
                        {#each { length: debugInfo.length }, index}
                        <svg width=12 height=12>
                            <circle cx=6 cy=6 r={index === currentPage ? 6 : 4} fill={index === currentPage ? "#1c75bc" : "black"} />
                        </svg>
                        {/each}
                    </div>
                    <svg width=16 height=16>
                        <polygon points="8,16 0,0 16,0" fill="black" />
                    </svg>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    code {
        font-family: "Courier New", Courier, monospace;
        font-weight: bold;
    }

    .debug-container {
        box-shadow: 0 0 20px rgba(0, 204, 255, 0.1);
        position: relative;
        overflow: hidden;
    }

    .debug-container::after {
        content: "";
        position: absolute;
        top: 0;
        left: -100%;
        width: 50%;
        height: 100%;
        background: linear-gradient(
            to right,
            rgba(0, 204, 255, 0),
            rgba(0, 204, 255, 0.1),
            rgba(0, 204, 255, 0)
        );
        animation: shimmer 3s infinite;
    }

    @keyframes shimmer {
        0% {
            left: -100%;
        }
        100% {
            left: 200%;
        }
    }

    .timer-display {
        border-radius: 8px;
        padding: 12px;
        background: rgba(0, 0, 0, 0.3);
        border: 1px solid rgba(0, 204, 255, 0.2);
        box-shadow: 0 0 15px rgba(0, 204, 255, 0.1);
    }

    .wonderlight-card {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        box-shadow: 0 0 20px rgba(28, 117, 188, 0.2);
    }

    .wonderlight-heading {
        color: #00ccff;
        text-shadow: 0 0 10px rgba(0, 204, 255, 0.5);
    }
</style>
