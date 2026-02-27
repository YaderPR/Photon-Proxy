<script>
    let { isRunning = false, isLoading = false, onToggle } = $props();
</script>

<div class="core-wrapper">
    <button
        class="core-button"
        class:active={isRunning}
        onclick={onToggle}
        disabled={isLoading}
        aria-label={isRunning ? "Disconnect Proxy" : "Initialize Proxy"}
    >
        <div class="glow-layer"></div>
        <div class="ring-outer"></div>
        <div class="ring-inner"></div>

        <div class="icon-container">
            <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
            >
                <path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path>
                <line x1="12" y1="2" x2="12" y2="12"></line>
            </svg>
        </div>
    </button>

    <div class="label-container">
        <span class="main-label"
            >{isLoading
                ? "SYNCING..."
                : isRunning
                  ? "DISCONNECT"
                  : "INITIALIZE"}</span
        >
        {#if isRunning}
            <span class="particle-status">PHOTON FLOW ACTIVE</span>
        {/if}
    </div>
</div>

<style>
    .core-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
        padding: 2rem;
    }

    .core-button {
        width: 160px;
        height: 160px;
        border-radius: 50%;
        background: #08080c;
        border: none;
        cursor: pointer;
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.5s cubic-bezier(0.16, 1, 0.3, 1);
        outline: none;
    }

    .glow-layer {
        position: absolute;
        inset: -20px;
        background: radial-gradient(circle, var(--purple) 0%, transparent 70%);
        opacity: 0.1;
        transition: all 0.5s;
        pointer-events: none;
    }

    .ring-outer {
        position: absolute;
        inset: 0;
        border: 4px solid var(--border);
        border-radius: 50%;
        transition: all 0.5s;
    }

    .ring-inner {
        position: absolute;
        inset: 12px;
        border: 2px solid var(--border);
        border-radius: 50%;
        opacity: 0.5;
        transition: all 0.5s;
    }

    .icon-container {
        width: 50px;
        height: 50px;
        color: var(--text-dim);
        opacity: 0.6;
        transition: all 0.5s;
        z-index: 2;
    }

    /* Interaction States */
    .core-button:hover:not(:disabled) {
        transform: scale(1.05);
    }

    .core-button:hover:not(:disabled) .ring-outer {
        border-color: rgba(255, 255, 255, 0.2);
        box-shadow: 0 0 30px rgba(255, 255, 255, 0.05);
    }

    /* Active State (Engine Running) */
    .core-button.active {
        background: #000;
    }

    .core-button.active .glow-layer {
        background: radial-gradient(circle, var(--cyan) 0%, transparent 70%);
        opacity: 0.25;
        inset: -30px;
    }

    .core-button.active .ring-outer {
        border-color: var(--cyan);
        box-shadow:
            0 0 40px var(--cyan-glow),
            inset 0 0 20px var(--cyan-glow);
        animation: pulse-glow 2s infinite ease-in-out;
    }

    .core-button.active .ring-inner {
        border-color: var(--cyan);
        opacity: 0.8;
        transform: scale(0.9);
    }

    .core-button.active .icon-container {
        color: var(--cyan);
        filter: drop-shadow(0 0 10px var(--cyan));
    }

    /* Labels */
    .label-container {
        text-align: center;
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }

    .main-label {
        font-size: 0.9rem;
        font-weight: 800;
        text-transform: uppercase;
        letter-spacing: 0.25rem;
        color: var(--text-dim);
        transition: color 0.5s;
    }

    .active + .label-container .main-label {
        color: var(--text);
    }

    .particle-status {
        font-family: var(--font-mono);
        font-size: 0.6rem;
        color: var(--cyan);
        letter-spacing: 0.1rem;
        opacity: 0.8;
        animation: flicker 2s infinite;
    }

    @keyframes flicker {
        0%,
        100% {
            opacity: 0.6;
        }
        50% {
            opacity: 1;
        }
    }

    @keyframes pulse-glow {
        0% {
            box-shadow:
                0 0 20px var(--cyan-glow),
                inset 0 0 10px var(--cyan-glow);
        }
        50% {
            box-shadow:
                0 0 40px var(--cyan-glow),
                inset 0 0 20px var(--cyan-glow);
        }
        100% {
            box-shadow:
                0 0 20px var(--cyan-glow),
                inset 0 0 10px var(--cyan-glow);
        }
    }
</style>
