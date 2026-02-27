<script>
    let { config = $bindable(), isRunning = false } = $props();
</script>

<section class="glass rounded-[20px] p-7 flex flex-col gap-6 h-full">
    <div class="flex items-center gap-3">
        <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-white/5 text-photon-cyan"
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
        <h3 class="text-[0.95rem] font-bold tracking-tight text-white m-0">
            Upstream Relay
        </h3>
    </div>

    <div class="flex flex-col gap-2.5">
        <label
            for="relay-protocol"
            class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
            >Protocol</label
        >
        <select
            id="relay-protocol"
            bind:value={config.upstream_type}
            disabled={isRunning}
            class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50"
        >
            <option value="direct">Direct Connection</option>
            <option value="socks5">SOCKS5 Tunnel</option>
            <option value="http">HTTP Connect</option>
        </select>
    </div>

    {#if config.upstream_type !== "direct"}
        <div
            class="flex flex-col gap-6 animate-in fade-in slide-in-from-bottom-2 duration-300"
        >
            <div class="flex gap-4">
                <div class="flex flex-[2] flex-col gap-2.5">
                    <label
                        for="remote-host"
                        class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
                        >Target Host</label
                    >
                    <input
                        id="remote-host"
                        type="text"
                        placeholder="e.g. proxy.com"
                        bind:value={config.upstream_host}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50 placeholder:text-white/20"
                    />
                </div>
                <div class="flex flex-1 flex-col gap-2.5">
                    <label
                        for="remote-port"
                        class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
                        >Port</label
                    >
                    <input
                        id="remote-port"
                        type="number"
                        bind:value={config.upstream_port}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50"
                    />
                </div>
            </div>

            <div class="flex gap-4">
                <div class="flex flex-1 flex-col gap-2.5">
                    <label
                        for="username"
                        class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
                        >Username</label
                    >
                    <input
                        id="username"
                        type="text"
                        placeholder="Optional"
                        bind:value={config.username}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50 placeholder:text-white/20"
                    />
                </div>
                <div class="flex flex-1 flex-col gap-2.5">
                    <label
                        for="password"
                        class="text-[0.7rem] font-semibold uppercase tracking-wider text-white/40"
                        >Password</label
                    >
                    <input
                        id="password"
                        type="password"
                        placeholder="Optional"
                        bind:value={config.password}
                        disabled={isRunning}
                        class="w-full rounded-xl border border-white/10 bg-black/40 p-3.5 text-sm text-white transition-all focus:border-photon-cyan focus:bg-white/5 focus:outline-none focus:ring-1 focus:ring-photon-cyan/30 disabled:opacity-50 placeholder:text-white/20"
                    />
                </div>
            </div>
        </div>
    {/if}
</section>
