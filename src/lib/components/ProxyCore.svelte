<script>
    let { isRunning = false, isLoading = false, onToggle } = $props();
</script>

<div class="flex flex-col items-center gap-8 p-6">
    <button
        class="group relative flex h-36 w-36 items-center justify-center rounded-full transition-all duration-500 ease-out outline-none disabled:cursor-not-allowed
          {isRunning ? 'bg-slate-900' : 'bg-slate-900/80'}"
        onclick={onToggle}
        disabled={isLoading}
        aria-label={isRunning ? "Disconnect Proxy" : "Initialize Proxy"}
    >
        <!-- Background Glow -->
        <div
            class="absolute -inset-4 rounded-full blur-2xl transition-all duration-700
              {isRunning ? 'bg-indigo-500/20 scale-110' : 'bg-slate-800/30'}"
        ></div>

        <!-- Outer Ring -->
        <div
            class="absolute inset-0 rounded-full border-[3px] transition-all duration-500 group-hover:shadow-lg
              {isRunning
                ? 'border-indigo-500/60 shadow-[0_0_30px_rgba(99,102,241,0.25)] animate-pulse-ring'
                : 'border-slate-700/50 group-hover:border-slate-600'}"
        ></div>

        <!-- Inner Ring -->
        <div
            class="absolute inset-4 rounded-full border-2 transition-all duration-500
              {isRunning
                ? 'border-indigo-400/40 scale-95'
                : 'border-slate-800'}"
        ></div>

        <!-- Icon -->
        <div
            class="z-10 h-10 w-10 transition-all duration-500
              {isRunning
                ? 'text-indigo-400 drop-shadow-[0_0_12px_rgba(129,140,248,0.6)]'
                : 'text-slate-500 group-hover:text-slate-400'}"
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
            class="text-[0.85rem] font-black uppercase tracking-[0.25em] transition-colors duration-500
              {isRunning ? 'text-slate-100' : 'text-slate-500'}"
        >
            {isLoading ? "SYNCING..." : isRunning ? "DISCONNECT" : "INITIALIZE"}
        </span>
        {#if isRunning}
            <span
                class="font-mono text-[0.6rem] tracking-[0.1em] text-indigo-400/70 animate-pulse"
            >
                PHOTON TUNNEL ACTIVE
            </span>
        {/if}
    </div>
</div>

<style>
    @keyframes pulse-ring {
        0%,
        100% {
            box-shadow: 0 0 15px rgba(99, 102, 241, 0.2);
        }
        50% {
            box-shadow: 0 0 35px rgba(99, 102, 241, 0.35);
        }
    }
    .animate-pulse-ring {
        animation: pulse-ring 2.5s infinite ease-in-out;
    }
</style>
