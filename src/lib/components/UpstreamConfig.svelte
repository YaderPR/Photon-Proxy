<script>
    let { config = $bindable(), isRunning = false } = $props();
</script>

<section class="glass rounded-2xl p-6 flex flex-col gap-5">
    <div class="flex items-center gap-3">
        <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-cyan-500/10 text-cyan-400"
        >
            <svg
                class="h-[18px] w-[18px]"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
            >
                <path d="M5 12h14M12 5l7 7-7 7"></path>
            </svg>
        </div>
        <h3 class="text-sm font-bold tracking-tight text-slate-200">
            Upstream Relay
        </h3>
    </div>

    <!-- Protocol -->
    <div class="flex flex-col gap-2">
        <label
            for="relay-protocol"
            class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
        >
            Protocol
        </label>
        <select
            id="relay-protocol"
            bind:value={config.upstream_type}
            disabled={isRunning}
            class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 text-sm text-slate-200 transition-all
              focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
              disabled:opacity-40 disabled:cursor-not-allowed appearance-none"
        >
            <option value="direct">Direct Connection</option>
            <option value="socks5">SOCKS5 Tunnel</option>
            <option value="http">HTTP Connect</option>
        </select>
    </div>

    {#if config.upstream_type !== "direct"}
        <div class="flex flex-col gap-4">
            <!-- Host & Port -->
            <div class="flex gap-3">
                <div class="flex flex-[2] flex-col gap-2">
                    <label
                        for="remote-host"
                        class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
                    >
                        Host
                    </label>
                    <input
                        id="remote-host"
                        type="text"
                        placeholder="e.g. proxy.example.com"
                        bind:value={config.upstream_host}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 text-sm text-slate-200 transition-all
                          focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
                          disabled:opacity-40 disabled:cursor-not-allowed placeholder:text-slate-700"
                    />
                </div>
                <div class="flex flex-1 flex-col gap-2">
                    <label
                        for="remote-port"
                        class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
                    >
                        Port
                    </label>
                    <input
                        id="remote-port"
                        type="number"
                        bind:value={config.upstream_port}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 text-sm text-slate-200 transition-all
                          focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
                          disabled:opacity-40 disabled:cursor-not-allowed"
                    />
                </div>
            </div>

            <!-- Credentials -->
            <div class="flex gap-3">
                <div class="flex flex-1 flex-col gap-2">
                    <label
                        for="username"
                        class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
                    >
                        Username
                    </label>
                    <input
                        id="username"
                        type="text"
                        placeholder="Optional"
                        bind:value={config.username}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 text-sm text-slate-200 transition-all
                          focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
                          disabled:opacity-40 disabled:cursor-not-allowed placeholder:text-slate-700"
                    />
                </div>
                <div class="flex flex-1 flex-col gap-2">
                    <label
                        for="password"
                        class="text-[0.65rem] font-semibold uppercase tracking-wider text-slate-500"
                    >
                        Password
                    </label>
                    <input
                        id="password"
                        type="password"
                        placeholder="Optional"
                        bind:value={config.password}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-slate-700/50 bg-slate-900/60 p-3 text-sm text-slate-200 transition-all
                          focus:border-indigo-500/50 focus:bg-slate-900 focus:outline-none focus:ring-1 focus:ring-indigo-500/20
                          disabled:opacity-40 disabled:cursor-not-allowed placeholder:text-slate-700"
                    />
                </div>
            </div>
        </div>
    {/if}
</section>
