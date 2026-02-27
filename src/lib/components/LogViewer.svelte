<script>
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import { Terminal, Shield, Globe, Activity, Trash2 } from "lucide-svelte";

    /** @type {any[]} */
    let logs = $state([]);
    let autoScroll = $state(true);
    /** @type {HTMLElement | undefined} */
    let logContainer = $state();

    $effect(() => {
        if (autoScroll && logContainer && logs.length) {
            logContainer.scrollTop = logContainer.scrollHeight;
        }
    });

    onMount(() => {
        const unlisten = listen("proxy-log", (event) => {
            logs = [...logs, event.payload].slice(-200); // Keep last 200 logs
        });

        return () => {
            unlisten.then((u) => u());
        };
    });

    function clearLogs() {
        logs = [];
    }

    function getProtocolColor(proto) {
        switch (proto?.toUpperCase()) {
            case "TCP":
                return "text-cyan-400";
            case "UDP":
                return "text-purple-400";
            case "DNS":
                return "text-yellow-400";
            default:
                return "text-slate-400";
        }
    }

    function getLevelColor(level) {
        switch (level?.toUpperCase()) {
            case "ERROR":
                return "text-red-400";
            case "WARN":
                return "text-orange-400";
            default:
                return "text-slate-400";
        }
    }
</script>

<div
    class="bg-slate-900/50 backdrop-blur-xl border border-white/10 rounded-2xl overflow-hidden flex flex-col h-[400px] shadow-2xl"
>
    <!-- Header -->
    <div
        class="px-4 py-3 border-b border-white/5 bg-white/5 flex items-center justify-between"
    >
        <div class="flex items-center gap-2">
            <div class="p-1.5 bg-cyan-500/20 rounded-lg">
                <Terminal class="w-4 h-4 text-cyan-400" />
            </div>
            <h3
                class="text-sm font-semibold text-white tracking-wide uppercase"
            >
                System Activity
            </h3>
        </div>

        <div class="flex items-center gap-4">
            <label class="flex items-center gap-2 cursor-pointer group">
                <input
                    type="checkbox"
                    bind:checked={autoScroll}
                    class="hidden"
                />
                <div
                    class="w-8 h-4 bg-slate-800 rounded-full relative transition-colors {autoScroll
                        ? 'bg-cyan-500/50'
                        : ''}"
                >
                    <div
                        class="absolute top-1 left-1 w-2 h-2 bg-white rounded-full transition-transform {autoScroll
                            ? 'translate-x-4'
                            : ''}"
                    ></div>
                </div>
                <span
                    class="text-xs text-slate-400 group-hover:text-white transition-colors"
                    >Auto-scroll</span
                >
            </label>

            <button
                onclick={clearLogs}
                class="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-slate-400 hover:text-red-400"
                title="Clear Logs"
            >
                <Trash2 class="w-4 h-4" />
            </button>
        </div>
    </div>

    <!-- Log Area -->
    <div
        bind:this={logContainer}
        class="flex-1 overflow-y-auto p-4 font-mono text-[11px] leading-relaxed scrollbar-thin scrollbar-thumb-white/10"
    >
        {#if logs.length === 0}
            <div
                class="h-full flex flex-col items-center justify-center text-slate-500 gap-3"
            >
                <Activity class="w-8 h-8 opacity-20" />
                <p>Awaiting network activity...</p>
            </div>
        {:else}
            <div class="space-y-1">
                {#each logs as log}
                    <div
                        class="group flex gap-3 py-0.5 hover:bg-white/5 rounded px-2 transition-colors border-l-2 border-transparent hover:border-cyan-500/50"
                    >
                        <span class="text-slate-600 shrink-0"
                            >{log.timestamp}</span
                        >
                        <span
                            class="font-bold w-8 shrink-0 {getProtocolColor(
                                log.protocol,
                            )}">{log.protocol}</span
                        >
                        <span class="flex-1 text-slate-300">
                            {log.message}
                            {#if log.src || log.dst}
                                <span class="text-slate-500 ml-2">
                                    {log.src}
                                    <span class="text-cyan-500/50">→</span>
                                    {log.dst}
                                </span>
                            {/if}
                        </span>
                        <span
                            class="text-[10px] uppercase font-bold px-1.5 rounded bg-white/5 {getLevelColor(
                                log.level,
                            )} self-center"
                        >
                            {log.level}
                        </span>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <!-- Footer Stats -->
    <div
        class="px-4 py-2 bg-black/20 border-t border-white/5 flex items-center justify-between text-[10px] text-slate-500"
    >
        <div class="flex gap-4">
            <span class="flex items-center gap-1">
                <Shield class="w-3 h-3 text-green-500/50" /> Encrypted Tunnel
            </span>
            <span class="flex items-center gap-1">
                <Globe class="w-3 h-3 text-blue-500/50" /> Global Traffic
            </span>
        </div>
        <span>{logs.length} events captured</span>
    </div>
</div>

<style>
    .scrollbar-thin::-webkit-scrollbar {
        width: 4px;
    }
    .scrollbar-thin::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 2px;
    }
    .scrollbar-thin::-webkit-scrollbar-track {
        background: transparent;
    }
</style>
