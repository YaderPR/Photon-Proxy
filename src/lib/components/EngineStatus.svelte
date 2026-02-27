<script>
    let { config = {}, isRunning = false } = $props();
</script>

<section class="glass rounded-2xl p-6 flex flex-col gap-5">
    <div class="flex items-center gap-3">
        <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-purple-500/10 text-purple-400"
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
        <h3 class="text-sm font-bold tracking-tight text-slate-200">
            Engine Status
        </h3>
    </div>

    <!-- Port Display -->
    <div class="flex flex-col gap-2">
        <label
            for="local-port"
            class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
        >
            Intercept Port
        </label>
        <div class="relative flex items-center">
            <input
                id="local-port"
                type="number"
                bind:value={config.local_port}
                disabled={isRunning}
                class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 font-mono text-sm text-slate-200 transition-all
                  focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
                  disabled:opacity-40 disabled:cursor-not-allowed"
            />
            <span
                class="absolute right-3 text-[0.6rem] font-bold tracking-widest text-slate-600"
                >TPROXY</span
            >
        </div>
    </div>

    <!-- Feature Indicators -->
    <div class="flex flex-col gap-2.5 mt-1">
        {#each [{ name: "Zero-Copy Splice Bridge", color: "cyan" }, { name: "DNS-over-TCP Tunnel", color: "purple" }, { name: "UDP Session Layer", color: "indigo" }] as feature}
            <div
                class="flex items-center gap-3 text-[0.72rem] transition-all duration-300
              {isRunning ? 'text-slate-300' : 'text-slate-600'}"
            >
                <span
                    class="h-1.5 w-1.5 rounded-full transition-all duration-500
                  {isRunning
                        ? feature.color === 'cyan'
                            ? 'bg-cyan-400 shadow-[0_0_6px_rgba(34,211,238,0.5)]'
                            : feature.color === 'purple'
                              ? 'bg-purple-400 shadow-[0_0_6px_rgba(168,85,247,0.5)]'
                              : 'bg-indigo-400 shadow-[0_0_6px_rgba(129,140,248,0.5)]'
                        : 'bg-slate-700'}"
                >
                </span>
                <span>{feature.name}</span>
            </div>
        {/each}
    </div>

    <!-- Running stats -->
    {#if isRunning}
        <div
            class="mt-auto border-t border-slate-700/30 pt-4 flex justify-between text-center"
        >
            <div class="flex flex-col gap-0.5">
                <span class="text-[0.75rem] font-bold text-slate-200"
                    >Zero-Copy</span
                >
                <span
                    class="text-[0.55rem] font-semibold text-slate-600 uppercase tracking-wider"
                    >Pipeline</span
                >
            </div>
            <div class="flex flex-col gap-0.5">
                <span class="text-[0.75rem] font-bold text-slate-200"
                    >AES-256</span
                >
                <span
                    class="text-[0.55rem] font-semibold text-slate-600 uppercase tracking-wider"
                    >Cipher</span
                >
            </div>
        </div>
    {/if}
</section>
