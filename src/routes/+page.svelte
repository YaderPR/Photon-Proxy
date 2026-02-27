<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  // Components
  import ProxyCore from "$lib/components/ProxyCore.svelte";
  import UpstreamConfig from "$lib/components/UpstreamConfig.svelte";
  import EngineStatus from "$lib/components/EngineStatus.svelte";
  import LogViewer from "$lib/components/LogViewer.svelte";

  // State
  let isRunning = $state(false);
  let isLoading = $state(false);
  let errorMsg = $state("");
  let activeTab = $state("dashboard");

  let config = $state({
    local_port: 1080,
    upstream_type: "direct",
    upstream_host: "",
    upstream_port: 1080,
    username: "",
    password: "",
  });

  const tabs = [
    { id: "dashboard", label: "Dashboard", icon: "power" },
    { id: "settings", label: "Settings", icon: "sliders" },
    { id: "logs", label: "Activity", icon: "terminal" },
  ];

  async function checkStatus() {
    try {
      isRunning = await invoke("get_proxy_status");
    } catch (e) {
      console.error("Status check failed:", e);
    }
  }

  async function toggleProxy() {
    isLoading = true;
    errorMsg = "";
    try {
      if (isRunning) {
        await invoke("stop_proxy");
      } else {
        const payload = {
          ...config,
          upstream_host: config.upstream_host || null,
          upstream_port: config.upstream_port || null,
          username: config.username || null,
          password: config.password || null,
        };
        await invoke("start_proxy", { config: payload });
      }
      setTimeout(checkStatus, 500);
    } catch (e) {
      errorMsg = String(e);
      console.error("Proxy operation failed:", e);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    checkStatus();
    const interval = setInterval(checkStatus, 2000);
    return () => clearInterval(interval);
  });
</script>

<div
  class="flex h-screen w-full overflow-hidden bg-photon-bg text-slate-200 selection:bg-photon-cyan/30"
>
  <!-- Sidebar -->
  <aside
    class="flex flex-col w-[76px] border-r border-slate-800 bg-photon-surface/80 backdrop-blur-lg"
  >
    <!-- Brand -->
    <div class="flex flex-col items-center gap-1.5 pt-7 pb-8">
      <div
        class="flex h-11 w-11 items-center justify-center rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/30 shadow-lg shadow-indigo-500/10"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          class="h-5 w-5 text-indigo-400"
        >
          <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
        </svg>
      </div>
      <span class="text-[0.5rem] font-extrabold tracking-[0.2em] text-slate-500"
        >PHOTON</span
      >
    </div>

    <!-- Nav Items -->
    <nav class="flex flex-1 flex-col items-center gap-1 px-2.5">
      {#each tabs as tab}
        <button
          class="flex flex-col items-center gap-1.5 w-full py-3 rounded-xl transition-all duration-200 cursor-pointer
            {activeTab === tab.id
            ? 'text-indigo-400 bg-indigo-500/10 border border-indigo-500/20 shadow-sm shadow-indigo-500/5'
            : 'text-slate-500 border border-transparent hover:text-slate-300 hover:bg-slate-800/50'}"
          onclick={() => (activeTab = tab.id)}
          title={tab.label}
        >
          {#if tab.icon === "power"}
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              class="h-[18px] w-[18px]"
            >
              <path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path>
              <line x1="12" y1="2" x2="12" y2="12"></line>
            </svg>
          {:else if tab.icon === "sliders"}
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              class="h-[18px] w-[18px]"
            >
              <line x1="4" y1="21" x2="4" y2="14"></line>
              <line x1="4" y1="10" x2="4" y2="3"></line>
              <line x1="12" y1="21" x2="12" y2="12"></line>
              <line x1="12" y1="8" x2="12" y2="3"></line>
              <line x1="20" y1="21" x2="20" y2="16"></line>
              <line x1="20" y1="12" x2="20" y2="3"></line>
              <line x1="1" y1="14" x2="7" y2="14"></line>
              <line x1="9" y1="8" x2="15" y2="8"></line>
              <line x1="17" y1="16" x2="23" y2="16"></line>
            </svg>
          {:else if tab.icon === "terminal"}
            <svg
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              class="h-[18px] w-[18px]"
            >
              <polyline points="4 17 10 11 4 5"></polyline>
              <line x1="12" y1="19" x2="20" y2="19"></line>
            </svg>
          {/if}
          <span class="text-[0.55rem] font-semibold tracking-wide"
            >{tab.label}</span
          >
        </button>
      {/each}
    </nav>

    <!-- Status Indicator at bottom -->
    <div class="flex flex-col items-center gap-2 pb-7 px-2">
      <div
        class="flex h-8 w-8 items-center justify-center rounded-full transition-all duration-500
        {isRunning
          ? 'bg-emerald-500/15 ring-1 ring-emerald-500/30'
          : 'bg-slate-800'}"
      >
        <span
          class="h-2.5 w-2.5 rounded-full transition-all duration-500
          {isRunning
            ? 'bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.5)]'
            : 'bg-slate-600'}"
        ></span>
      </div>
      <span
        class="text-[0.5rem] font-bold tracking-widest transition-colors duration-500 {isRunning
          ? 'text-emerald-400'
          : 'text-slate-600'}"
      >
        {isRunning ? "ACTIVE" : "IDLE"}
      </span>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex flex-1 flex-col overflow-hidden bg-photon-bg">
    <!-- Top Bar -->
    <header
      class="flex items-center justify-between border-b border-slate-800 bg-photon-surface/40 backdrop-blur-sm px-7 py-4"
    >
      <div class="flex items-center gap-4">
        <h1 class="text-[0.95rem] font-bold tracking-wide text-slate-100">
          {tabs.find((t) => t.id === activeTab)?.label}
        </h1>
        <div class="hidden sm:flex items-center gap-2">
          <div class="h-1 w-1 rounded-full bg-slate-700"></div>
          <span
            class="text-[0.65rem] font-medium tracking-wider text-slate-600"
          >
            Photon Proxy Engine
          </span>
        </div>
      </div>

      <div
        class="flex items-center gap-2.5 rounded-full border px-4 py-2 text-[0.6rem] font-bold tracking-widest transition-all duration-500
        {isRunning
          ? 'border-emerald-500/25 bg-emerald-500/5 text-emerald-400'
          : 'border-slate-700 bg-slate-800/50 text-slate-500'}"
      >
        <span class="relative flex h-1.5 w-1.5">
          {#if isRunning}
            <span
              class="absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75 animate-ping"
            ></span>
          {/if}
          <span
            class="relative inline-flex h-1.5 w-1.5 rounded-full {isRunning
              ? 'bg-emerald-400'
              : 'bg-slate-600'}"
          ></span>
        </span>
        {isRunning ? "PROTECTED" : "UNPROTECTED"}
      </div>
    </header>

    <!-- Content Area -->
    <div class="flex-1 overflow-y-auto p-7">
      <!-- Error Toast -->
      {#if errorMsg}
        <div
          class="flex items-center gap-4 rounded-xl border border-red-500/20 bg-red-950/30 p-4 mb-6"
        >
          <svg
            class="h-5 w-5 text-red-400 shrink-0"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          <div class="flex flex-col gap-0.5 flex-1 min-w-0">
            <span
              class="text-[0.7rem] font-extrabold tracking-wider text-red-400"
              >ERROR</span
            >
            <span class="text-xs text-slate-300 break-words">{errorMsg}</span>
          </div>
          <button
            class="text-slate-500 hover:text-slate-200 text-lg transition-colors"
            onclick={() => (errorMsg = "")}>&times;</button
          >
        </div>
      {/if}

      <!-- Dashboard View -->
      {#if activeTab === "dashboard"}
        <div class="flex flex-col items-center gap-10">
          <ProxyCore {isRunning} {isLoading} onToggle={toggleProxy} />
          <div class="w-full max-w-md">
            <EngineStatus {config} {isRunning} />
          </div>
        </div>

        <!-- Settings View -->
      {:else if activeTab === "settings"}
        <div class="mx-auto max-w-lg">
          <UpstreamConfig bind:config {isRunning} />
        </div>

        <!-- Logs View -->
      {:else if activeTab === "logs"}
        <div class="h-full">
          <LogViewer />
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <footer
      class="flex items-center justify-between border-t border-slate-800/50 px-7 py-3 text-[0.6rem] font-medium tracking-widest text-slate-700 font-mono"
    >
      <span>v1.2.4</span>
      <span class="hidden sm:block">PHOTON ENGINE</span>
      <span>{isRunning ? "● ONLINE" : "○ OFFLINE"}</span>
    </footer>
  </main>
</div>

<style>
  @reference "../app.css";

  :global(.glass) {
    background: rgba(20, 20, 35, 0.8);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(100, 116, 139, 0.15);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.3);
  }
</style>
