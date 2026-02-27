<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  // Components
  import ProxyCore from "$lib/components/ProxyCore.svelte";
  import UpstreamConfig from "$lib/components/UpstreamConfig.svelte";
  import EngineStatus from "$lib/components/EngineStatus.svelte";
  import LogViewer from "$lib/components/LogViewer.svelte";

  // State management using Svelte 5 runes
  let isRunning = $state(false);
  let isLoading = $state(false);
  let errorMsg = $state("");

  let config = $state({
    local_port: 1080,
    upstream_type: "direct",
    upstream_host: "",
    upstream_port: 1080,
    username: "",
    password: "",
  });

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
        // Sanitize payload
        const payload = {
          ...config,
          upstream_host: config.upstream_host || null,
          upstream_port: config.upstream_port || null,
          username: config.username || null,
          password: config.password || null,
        };
        await invoke("start_proxy", { config: payload });
      }
      // Immediate check after toggle
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
  class="relative flex min-h-screen w-full flex-col overflow-x-hidden bg-photon-bg text-white selection:bg-photon-cyan/30"
>
  <!-- Background Effects -->
  <div class="fixed inset-0 -z-10 overflow-hidden pointer-events-none">
    <div
      class="absolute top-[-20%] left-1/2 h-[1000px] w-[1000px] -translate-x-1/2 rounded-full bg-photon-purple/20 blur-[120px]"
    ></div>
    <div
      class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:40px_40px] [mask-image:linear-gradient(to_bottom,black,transparent)]"
    ></div>
  </div>

  <!-- Navigation -->
  <nav class="flex w-full items-center justify-between px-8 py-10 lg:px-12">
    <div class="text-xl font-extrabold tracking-[0.3em]">
      <span class="text-photon-purple">P</span>HOTON
      <span class="text-photon-cyan drop-shadow-[0_0_15px_rgba(0,242,255,0.4)]"
        >PROXY</span
      >
    </div>
    <div
      class="flex items-center gap-3 rounded-full border border-white/10 bg-white/5 px-5 py-2 text-[0.7rem] font-bold tracking-widest transition-all duration-500 {isRunning
        ? 'active-pill'
        : ''}"
    >
      <span class="relative flex h-2 w-2">
        <span
          class="absolute inline-flex h-full w-full rounded-full opacity-75 {isRunning
            ? 'animate-ping bg-photon-cyan'
            : 'bg-white/20'}"
        ></span>
        <span
          class="relative inline-flex h-2 w-2 rounded-full {isRunning
            ? 'bg-photon-cyan'
            : 'bg-white/20'}"
        ></span>
      </span>
      <span class={isRunning ? "text-photon-cyan" : "text-white/40"}>
        {isRunning ? "ENCRYPTED" : "UNPROTECTED"}
      </span>
    </div>
  </nav>

  <!-- Main Content -->
  <main
    class="mx-auto flex w-full max-w-5xl flex-1 flex-col items-center px-6 pb-16"
  >
    <div class="mb-12">
      <ProxyCore {isRunning} {isLoading} onToggle={toggleProxy} />
    </div>

    {#if errorMsg}
      <div
        class="relative mb-8 flex w-full max-w-lg items-center gap-5 rounded-2xl border-l-4 border-red-500 bg-red-500/10 p-4 backdrop-blur-md"
      >
        <svg
          class="h-6 w-6 text-red-500 shrink-0"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        <div class="flex flex-col gap-0.5">
          <span
            class="text-[0.75rem] font-extrabold tracking-wider text-red-500"
            >EXECUTION ERROR</span
          >
          <span class="text-sm text-white/90">{errorMsg}</span>
        </div>
        <button
          class="ml-auto text-xl text-white/40 hover:text-white"
          onclick={() => (errorMsg = "")}>&times;</button
        >
      </div>
    {/if}

    <div class="grid w-full grid-cols-1 gap-8 md:grid-cols-2">
      <UpstreamConfig bind:config {isRunning} />
      <EngineStatus {config} {isRunning} />
    </div>

    <!-- Logs Section -->
    <div class="w-full mt-12">
      <LogViewer />
    </div>
  </main>

  <!-- Footer -->
  <footer
    class="flex w-full items-center justify-between border-t border-white/10 px-8 py-6 text-[0.65rem] font-semibold tracking-widest text-white/20 lg:px-12 font-mono"
  >
    <div>v1.2.4-STABLE</div>
    <div class="hidden sm:block">PHOTON QUANTUM INTERFACE</div>
    <div>LATENCY: -- MS</div>
  </footer>
</div>

<style>
  @reference "../app.css";
  .active-pill {
    @apply border-photon-cyan bg-photon-cyan/10 text-photon-cyan shadow-[0_0_20px_rgba(0,242,255,0.2)];
  }
</style>
