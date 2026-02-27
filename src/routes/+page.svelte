<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  // Components
  import ProxyCore from "$lib/components/ProxyCore.svelte";
  import UpstreamConfig from "$lib/components/UpstreamConfig.svelte";
  import EngineStatus from "$lib/components/EngineStatus.svelte";

  // Global Theme
  import "$lib/styles/theme.css";

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

<div class="app-layout">
  <div class="background-effects">
    <div class="gradient-sphere"></div>
    <div class="grid-overlay"></div>
  </div>

  <nav class="top-nav animate-fade">
    <div class="brand">
      <span class="p">P</span>HOTON <span class="accent">PROXY</span>
    </div>
    <div class="status-pill" class:active={isRunning}>
      <span class="pulse-dot"></span>
      {isRunning ? "ENCRYPTED" : "UNPROTECTED"}
    </div>
  </nav>

  <main class="content-wrapper">
    <div class="core-engagement animate-pop">
      <ProxyCore {isRunning} {isLoading} onToggle={toggleProxy} />
    </div>

    {#if errorMsg}
      <div class="error-toast glass animate-slide-up">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" y1="8" x2="12" y2="12"></line>
          <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        <div class="error-content">
          <span class="error-title">EXECUTION ERROR</span>
          <span class="error-desc">{errorMsg}</span>
        </div>
        <button class="close-error" onclick={() => (errorMsg = "")}
          >&times;</button
        >
      </div>
    {/if}

    <div class="dashboard-grid animate-fade-delayed">
      <UpstreamConfig bind:config {isRunning} />
      <EngineStatus {config} {isRunning} />
    </div>
  </main>

  <footer class="bottom-bar">
    <div class="footer-left">v1.2.4-STABLE</div>
    <div class="footer-center">PHOTON QUANTUM INTERFACE</div>
    <div class="footer-right">LATENCY: -- MS</div>
  </footer>
</div>

<style>
  .app-layout {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: 1;
  }

  /* Background Aesthetics */
  .background-effects {
    position: fixed;
    inset: 0;
    z-index: -1;
    overflow: hidden;
  }

  .gradient-sphere {
    position: absolute;
    top: -20%;
    left: 50%;
    transform: translateX(-50%);
    width: 1000px;
    height: 1000px;
    background: radial-gradient(circle, var(--purple-glow) 0%, transparent 70%);
    filter: blur(100px);
    opacity: 0.3;
  }

  .grid-overlay {
    position: absolute;
    inset: 0;
    background-image: linear-gradient(
        rgba(255, 255, 255, 0.02) 1px,
        transparent 1px
      ),
      linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
    background-size: 40px 40px;
    mask-image: linear-gradient(to bottom, black, transparent);
  }

  /* Nav */
  .top-nav {
    padding: 2.5rem 3rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .brand {
    font-size: 1.25rem;
    font-weight: 800;
    letter-spacing: 0.3rem;
    color: var(--text);
  }

  .brand .p {
    color: var(--purple);
  }
  .brand .accent {
    color: var(--cyan);
    text-shadow: 0 0 15px var(--cyan-glow);
  }

  .status-pill {
    padding: 0.6rem 1.2rem;
    border-radius: 40px;
    font-size: 0.7rem;
    font-weight: 800;
    letter-spacing: 0.15rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--text-dim);
    transition: all 0.5s;
  }

  .status-pill.active {
    background: rgba(0, 242, 255, 0.08);
    border-color: var(--cyan);
    color: var(--cyan);
    box-shadow: 0 0 20px var(--cyan-glow);
  }

  .pulse-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--text-vdim);
  }

  .active .pulse-dot {
    background: var(--cyan);
    animation: pulse-ring 2s infinite;
  }

  @keyframes pulse-ring {
    0% {
      transform: scale(0.8);
      opacity: 0.5;
    }
    50% {
      transform: scale(1.2);
      opacity: 1;
      box-shadow: 0 0 10px var(--cyan);
    }
    100% {
      transform: scale(0.8);
      opacity: 0.5;
    }
  }

  /* Main Content */
  .content-wrapper {
    flex: 1;
    max-width: 1000px;
    margin: 0 auto;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-bottom: 4rem;
    overflow-y: auto;
  }

  .core-engagement {
    margin-bottom: 3rem;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    width: 100%;
  }

  /* Error Toast */
  .error-toast {
    max-width: 500px;
    width: 100%;
    padding: 1rem 1.5rem;
    border-radius: 16px;
    border-left: 4px solid var(--error);
    margin-bottom: 2rem;
    display: flex;
    align-items: center;
    gap: 1.25rem;
    position: relative;
    background: rgba(255, 77, 77, 0.05);
  }

  .error-toast svg {
    width: 24px;
    color: var(--error);
    flex-shrink: 0;
  }

  .error-content {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .error-title {
    font-size: 0.75rem;
    font-weight: 800;
    color: var(--error);
    letter-spacing: 0.1rem;
  }

  .error-desc {
    font-size: 0.85rem;
    color: var(--text);
    opacity: 0.9;
  }

  .close-error {
    background: none;
    border: none;
    color: var(--text-vdim);
    font-size: 1.5rem;
    cursor: pointer;
    margin-left: auto;
  }

  /* Footer */
  .bottom-bar {
    padding: 1.5rem 3rem;
    display: flex;
    justify-content: space-between;
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    color: var(--text-vdim);
    letter-spacing: 0.1rem;
    border-top: 1px solid var(--border);
  }

  /* Animations */
  .animate-fade {
    animation: fadeIn 0.8s ease-out;
  }
  .animate-fade-delayed {
    animation: fadeIn 1s ease-out both;
    animation-delay: 0.3s;
  }
  .animate-pop {
    animation: popIn 0.6s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .animate-slide-up {
    animation: slideUp 0.4s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  @keyframes popIn {
    from {
      transform: scale(0.9);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  /* Responsive */
  @media (max-width: 800px) {
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
    .content-wrapper {
      padding: 0 1.5rem;
    }
    .top-nav {
      padding: 2rem 1.5rem;
    }
  }
</style>
