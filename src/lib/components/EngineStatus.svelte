<script>
    let { config = {}, isRunning = false } = $props();
</script>

<section class="glass rounded-[20px] p-7 flex flex-col gap-6 h-full">
    <div class="flex items-center gap-3">
        <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-white/5 text-photon-purple"
        >
            <svg
                class="h-[18px] w-[18px]"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
            >
                <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                <line x1="6" y1="6" x2="6.01" y2="6"></line>
                <line x1="6" y1="18" x2="6.01" y2="18"></line>
            </svg>
        </div>
        <h3 class="text-[0.95rem] font-bold tracking-tight text-white m-0">
            Engine Matrix
        </h3>
    </div>

    <div class="flex flex-col gap-2.5">
        <label
            for="local-port"
            class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
            >Intercept Port</label
        >
        <div class="relative flex items-center">
            <input
                id="local-port"
                type="number"
                bind:value={config.local_port}
                disabled={isRunning}
                class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 font-mono text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50"
            />
            <span
                class="absolute right-4 text-[0.65rem] font-extrabold tracking-widest text-white/20"
                >TPROXY</span
            >
        </div>
    </div>

    <div class="mt-2 flex flex-col gap-3">
        {#each ["Zero-Copy Splice Bridge", "DNS-over-TCP Secure Tunnel", "Transparent UDP Session Layer"] as feature}
            <div
                class="flex items-center gap-3 text-[0.75rem] transition-all duration-300 {isRunning
                    ? 'text-photon-cyan'
                    : 'text-white/40'}"
            >
                <span
                    class="h-1.5 w-1.5 rounded-full transition-all duration-300 {isRunning
                        ? 'bg-photon-cyan shadow-[0_0_10px_rgba(0,242,255,1)]'
                        : 'bg-white/10'}"
                ></span>
                <span>{feature}</span>
            </div>
        {/each}
    </div>

    {#if isRunning}
        <div
            class="mt-auto border-t border-white/10 pt-6 flex justify-between animate-in fade-in duration-500"
        >
            <div class="flex flex-col gap-1">
                <span
                    class="text-[0.8rem] font-extrabold tracking-widest text-white uppercase"
                    >Fast</span
                >
                <span
                    class="text-[0.6rem] font-semibold text-white/40 uppercase"
                    >Pipeline</span
                >
            </div>
            <div class="flex flex-col gap-1 text-right">
                <span
                    class="text-[0.8rem] font-extrabold tracking-widest text-white uppercase"
                    >AES-GCM</span
                >
                <span
                    class="text-[0.6rem] font-semibold text-white/40 uppercase"
                    >Cipher</span
                >
            </div>
        </div>
    {/if}
</section>
