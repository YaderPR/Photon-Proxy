<script>
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";

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
            logs = [...logs, event.payload].slice(-200);
        });
        return () => {
            unlisten.then((u) => u());
        };
    });

    function clearLogs() {
        logs = [];
    }

    /** @param {string} proto */
    function getProtocolColor(proto) {
        switch (proto?.toUpperCase()) {
            case "TCP":
                return "text-cyan-400";
            case "UDP":
                return "text-purple-400";
            case "DNS":
                return "text-amber-400";
            default:
                return "text-slate-500";
        }
    }

    /** @param {string} level */
    function getLevelColor(level) {
        switch (level?.toUpperCase()) {
            case "ERROR":
                return "text-red-400 bg-red-500/10";
            case "WARN":
                return "text-amber-400 bg-amber-500/10";
            default:
                return "text-slate-500 bg-slate-800/50";
        }
    }
</script>

<div class="glass rounded-2xl overflow-hidden flex flex-col h-full">
    <!-- Header -->
    <div
        class="px-5 py-3 border-b border-slate-700/30 flex items-center justify-between"
    >
        <div class="flex items-center gap-2.5">
            <div class="p-1.5 bg-indigo-500/10 rounded-lg">
                <svg
                    class="w-4 h-4 text-indigo-400"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <polyline points="4 17 10 11 4 5"></polyline>
                    <line x1="12" y1="19" x2="20" y2="19"></line>
                </svg>
            </div>
            <h3
                class="text-[0.8rem] font-semibold text-slate-300 tracking-wide"
            >
                System Activity
            </h3>
            <span class="text-[0.6rem] text-slate-600 font-mono"
                >{logs.length} events</span
            >
        </div>

        <div class="flex items-center gap-3">
            <label class="flex items-center gap-2 cursor-pointer group">
                <input
                    type="checkbox"
                    bind:checked={autoScroll}
                    class="hidden"
                />
                <div
                    class="w-7 h-3.5 bg-slate-800 rounded-full relative transition-colors {autoScroll
                        ? 'bg-indigo-500/40'
                        : ''}"
                >
                    <div
                        class="absolute top-0.5 left-0.5 w-2.5 h-2.5 bg-slate-400 rounded-full transition-all {autoScroll
                            ? 'translate-x-3 bg-indigo-400'
                            : ''}"
                    ></div>
                </div>
                <span
                    class="text-[0.6rem] text-slate-500 group-hover:text-slate-300 transition-colors"
                    >Auto</span
                >
            </label>

            <button
                onclick={clearLogs}
                class="p-1.5 hover:bg-slate-800 rounded-lg transition-colors text-slate-600 hover:text-red-400"
                title="Clear Logs"
            >
                <svg
                    class="w-3.5 h-3.5"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path
                        d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"
                    ></path>
                </svg>
            </button>
        </div>
    </div>

    <!-- Log Area -->
    <div
        bind:this={logContainer}
        class="flex-1 overflow-y-auto p-3 font-mono text-[11px] leading-relaxed log-scroll"
    >
        {#if logs.length === 0}
            <div
                class="h-full flex flex-col items-center justify-center text-slate-600 gap-3"
            >
                <svg
                    class="w-8 h-8 opacity-30"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                >
                    <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"
                    ></polyline>
                </svg>
                <p class="text-[0.7rem]">Awaiting network activity...</p>
            </div>
        {:else}
            <div class="space-y-px">
                {#each logs as log}
                    <div
                        class="group flex gap-3 py-1 px-2 rounded-md hover:bg-slate-800/40 transition-colors"
                    >
                        <span class="text-slate-700 shrink-0 tabular-nums"
                            >{log.timestamp}</span
                        >
                        <span
                            class="font-bold w-7 shrink-0 {getProtocolColor(
                                log.protocol,
                            )}">{log.protocol}</span
                        >
                        <span class="flex-1 text-slate-400">
                            {log.message}
                            {#if log.src || log.dst}
                                <span class="text-slate-600 ml-1.5">
                                    {log.src}
                                    <span class="text-indigo-500/50 mx-0.5"
                                        >→</span
                                    >
                                    {log.dst}
                                </span>
                            {/if}
                        </span>
                        <span
                            class="text-[9px] uppercase font-bold px-1.5 py-0.5 rounded {getLevelColor(
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
</div>

<style>
    .log-scroll::-webkit-scrollbar {
        width: 4px;
    }
    .log-scroll::-webkit-scrollbar-thumb {
        background: rgba(100, 116, 139, 0.2);
        border-radius: 2px;
    }
    .log-scroll::-webkit-scrollbar-track {
        background: transparent;
    }
</style>
