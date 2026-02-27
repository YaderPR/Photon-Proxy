<script>
    let { isRunning = false, isLoading = false, onToggle } = $props();
</script>

<div class="flex flex-col items-center gap-8 p-8">
    <button
        class="group relative flex h-40 w-40 items-center justify-center rounded-full bg-[#08080c] transition-all duration-500 ease-out outline-none disabled:cursor-not-allowed"
        class:active-core={isRunning}
        onclick={onToggle}
        disabled={isLoading}
        aria-label={isRunning ? "Disconnect Proxy" : "Initialize Proxy"}
    >
        <!-- Background Glow -->
        <div
            class="absolute -inset-5 rounded-full blur-2xl transition-all duration-500 {isRunning
                ? 'bg-photon-cyan/25 scale-125'
                : 'bg-photon-purple/10'}"
        ></div>

        <!-- Rings -->
        <div
            class="absolute inset-0 rounded-full border-4 border-white/5 transition-all duration-500 group-hover:border-white/10 group-hover:shadow-[0_0_30px_rgba(255,255,255,0.05)] {isRunning
                ? 'border-photon-cyan shadow-[0_0_40px_rgba(0,242,255,0.2),inset_0_0_20px_rgba(0,242,255,0.2)] animate-pulse-glow'
                : ''}"
        ></div>

        <div
            class="absolute inset-4 rounded-full border-2 border-white/5 opacity-50 transition-all duration-500 {isRunning
                ? 'border-photon-cyan opacity-80 scale-90'
                : ''}"
        ></div>

        <!-- Icon -->
        <div
            class="z-10 h-12 w-12 transition-all duration-500 {isRunning
                ? 'text-photon-cyan drop-shadow-[0_0_10px_rgba(0,242,255,0.8)] opacity-100'
                : 'text-white/40'}"
        >
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

    <div class="flex flex-col items-center gap-2 text-center">
        <span
            class="text-[0.9rem] font-black uppercase tracking-[0.25em] transition-colors duration-500 {isRunning
                ? 'text-white'
                : 'text-white/40'}"
        >
            {isLoading ? "SYNCING..." : isRunning ? "DISCONNECT" : "INITIALIZE"}
        </span>
        {#if isRunning}
            <span
                class="font-mono text-[0.6rem] tracking-[0.1em] text-photon-cyan opacity-80 animate-pulse"
            >
                PHOTON FLOW ACTIVE
            </span>
        {/if}
    </div>
</div>

<style>
    @reference "../../app.css";
    .active-core {
        @apply bg-black;
    }

    @keyframes pulse-glow {
        0%,
        100% {
            box-shadow:
                0 0 20px rgba(0, 242, 255, 0.2),
                inset 0 0 10px rgba(0, 242, 255, 0.2);
        }
        50% {
            box-shadow:
                0 0 40px rgba(0, 242, 255, 0.4),
                inset 0 0 20px rgba(0, 242, 255, 0.4);
        }
    }

    .animate-pulse-glow {
        animation: pulse-glow 2s infinite ease-in-out;
    }
</style>
