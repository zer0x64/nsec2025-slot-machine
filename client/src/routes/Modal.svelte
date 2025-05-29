<script lang="ts">
    import { modalStore, closeModal, type ModalData } from "$lib/modalStore";
    import { fade, fly } from "svelte/transition";
    import { onMount, onDestroy } from "svelte";
    import { gameEnded } from "./sharedState.svelte";

    // For direct usage as a component (access card modal)
    let { open = false, title = "", children = undefined } = $props();

    // For handling keydown events (close on escape)
    function handleKeydown(event: KeyboardEvent) {
        // If game has ended, only allow badge removal
        if (gameEnded.value) {
            return;
        }

        if (event.key === "Escape") {
            if ($modalStore) {
                closeModal();
            }
        }
        // Close on Space or Enter key press
        if (
            (event.key === " " || event.key === "Enter") &&
            $modalStore?.showCloseButton &&
            !gameEnded.value
        ) {
            closeModal();
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });

    // Helper function to get the appropriate icon for each modal type
    function getModalIcon(type: ModalData["type"]) {
        switch (type) {
            case "success":
                return "✓";
            case "error":
                return "✕";
            case "info":
                return "ℹ";
            case "flag":
                return "🏆";
            default:
                return "";
        }
    }

    // Helper function to get the appropriate color class for each modal type
    function getModalColorClass(type: ModalData["type"]) {
        switch (type) {
            case "success":
                return "success-modal";
            case "error":
                return "error-modal";
            case "info":
                return "info-modal";
            case "flag":
                return "flag-modal";
            default:
                return "";
        }
    }

    // Custom message for game end states
    function getModalInstructionText() {
        if (gameEnded.value) {
            return "Remove access card to exit";
        }
        return "Press action button to continue";
    }
</script>

<!-- Component-based Modal (for access card) -->
{#if open}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }}>
        <div class="modal-container" transition:fly={{ y: -20, duration: 300 }}>
            <div class="modal-content standard-modal">
                <!-- Modal header -->
                {#if title}
                    <div class="modal-header">
                        <h3 class="modal-title">
                            {title}
                        </h3>
                    </div>
                {/if}
                <!-- Modal body -->
                <div class="modal-body" role="document">
                    {#if children}
                        {@render children()}
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<!-- Store-based Modal (for alerts) -->
{#if $modalStore}
    <div class="modal-backdrop" transition:fade={{ duration: 200 }}>
        <div class="modal-container" transition:fly={{ y: -20, duration: 300 }}>
            <div
                class={`modal-content ${getModalColorClass($modalStore.type)}`}
            >
                <!-- Modal header -->
                <div class="modal-header">
                    <div class="modal-icon">
                        {getModalIcon($modalStore.type)}
                    </div>
                    <h3 class="modal-title">
                        {$modalStore.title}
                    </h3>
                </div>

                <!-- Modal body -->
                <div class="modal-body">
                    {#if $modalStore.header}
                        <p class="text-xl text-white mb-8" style="white-space: pre-line">{$modalStore.header}</p>
                    {/if}
                    <p class="modal-message">{$modalStore.message}</p>
                </div>

                <!-- Instructions instead of a button -->
                {#if $modalStore.showCloseButton}
                    <div class="modal-footer">
                        <div class="modal-instruction">
                            {getModalInstructionText()}
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(10, 40, 68, 0.8);
        backdrop-filter: blur(3px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal-container {
        max-width: 95vw;
        width: 800px; /* Increased width to fit longer content */
        max-height: 90vh;
        overflow: auto;
    }

    .modal-content {
        border-radius: 12px;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        border: 1px solid rgba(0, 204, 255, 0.3);
        box-shadow: 0 0 30px rgba(0, 204, 255, 0.3);
        background: rgba(10, 40, 68, 0.95);
        position: relative;
    }

    .modal-content::before {
        content: "";
        position: absolute;
        top: -10px;
        left: -10px;
        right: -10px;
        bottom: -10px;
        background: linear-gradient(
            45deg,
            transparent,
            rgba(0, 204, 255, 0.1),
            transparent
        );
        z-index: -1;
        animation: shimmer 2s infinite linear;
    }

    .modal-header {
        padding: 1.25rem;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        border-bottom: 1px solid rgba(0, 204, 255, 0.2);
    }

    .modal-icon {
        font-size: 1.5rem;
        color: #00ccff;
        width: 2rem;
        height: 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        background: rgba(0, 204, 255, 0.1);
    }

    .modal-title {
        font-size: 1.5rem;
        font-weight: bold;
        color: #00ccff;
        text-shadow: 0 0 10px rgba(0, 204, 255, 0.3);
        margin: 0;
    }

    .modal-body {
        padding: 1.25rem;
        overflow-y: auto;
    }

    .modal-message {
        font-size: 1.1rem;
        line-height: 1.5;
        color: white;
        word-wrap: break-word; /* Enable word wrapping */
        overflow-wrap: break-word; /* Ensure long words break */
    }

    .modal-footer {
        padding: 1rem 1.25rem;
        display: flex;
        justify-content: flex-end;
        gap: 0.5rem;
        border-top: 1px solid rgba(0, 204, 255, 0.2);
    }

    /* New style for key instruction */
    .modal-instruction {
        text-align: center;
        padding: 0.5rem 1.25rem;
        font-size: 1.1rem;
        color: #7fdbff;
        width: 100%;
        font-weight: bold;
    }

    /* Modal type-specific styles */
    .success-modal .modal-icon {
        background-color: rgba(0, 200, 83, 0.1);
        color: #00c853;
    }

    .success-modal .modal-title {
        color: #00c853;
        text-shadow: 0 0 10px rgba(0, 200, 83, 0.3);
    }

    .error-modal .modal-icon {
        background-color: rgba(255, 65, 54, 0.1);
        color: #ff4136;
    }

    .error-modal .modal-title {
        color: #ff4136;
        text-shadow: 0 0 10px rgba(255, 65, 54, 0.3);
    }

    .info-modal .modal-icon {
        background-color: rgba(0, 116, 217, 0.1);
        color: #0074d9;
    }

    .info-modal .modal-title {
        color: #0074d9;
        text-shadow: 0 0 10px rgba(0, 116, 217, 0.3);
    }

    .flag-modal .modal-icon {
        background-color: rgba(255, 195, 0, 0.1);
        color: #ffcc00;
        font-size: 1.75rem;
    }

    .flag-modal .modal-title {
        color: #ffcc00;
        text-shadow: 0 0 10px rgba(255, 220, 0, 0.5);
    }

    .flag-modal .modal-message {
        font-family: monospace;
        font-size: 1.2rem; /* Slightly smaller font size to fit more text */
        text-align: center;
        padding: 1rem;
        margin: 0.5rem;
        background: rgba(0, 0, 0, 0.3);
        border-radius: 8px;
        color: #ffcc00;
        overflow-x: auto; /* Allow horizontal scrolling if needed */
        white-space: pre-wrap; /* Preserve whitespace but wrap */
        word-break: break-word; /* Break long words if necessary */
    }

    /* Animation */
    @keyframes shimmer {
        0% {
            transform: translateX(-100%);
        }
        100% {
            transform: translateX(100%);
        }
    }
</style>
