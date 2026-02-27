<script>
    let { config = $bindable(), isRunning = false } = $props();
</script>

<section class="card glass">
    <div class="card-header">
        <div class="icon-tag">
            <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
            >
                <path d="M5 12h14M12 5l7 7-7 7"></path>
            </svg>
        </div>
        <h3>Upstream Relay</h3>
    </div>

    <div class="field">
        <label for="relay-protocol">Protocol</label>
        <div class="select-wrapper">
            <select
                id="relay-protocol"
                bind:value={config.upstream_type}
                disabled={isRunning}
            >
                <option value="direct">Direct Connection</option>
                <option value="socks5">SOCKS5 Tunnel</option>
                <option value="http">HTTP Connect</option>
            </select>
        </div>
    </div>

    {#if config.upstream_type !== "direct"}
        <div class="config-inner animate-in">
            <div class="row">
                <div class="field flex-2">
                    <label for="remote-host">Target Host</label>
                    <input
                        id="remote-host"
                        type="text"
                        placeholder="e.g. proxy.server.com"
                        bind:value={config.upstream_host}
                        disabled={isRunning}
                    />
                </div>
                <div class="field">
                    <label for="remote-port">Port</label>
                    <input
                        id="remote-port"
                        type="number"
                        bind:value={config.upstream_port}
                        disabled={isRunning}
                    />
                </div>
            </div>

            <div class="row">
                <div class="field">
                    <label for="username">Username</label>
                    <input
                        id="username"
                        type="text"
                        placeholder="Optional"
                        bind:value={config.username}
                        disabled={isRunning}
                    />
                </div>
                <div class="field">
                    <label for="password">Password</label>
                    <input
                        id="password"
                        type="password"
                        placeholder="Optional"
                        bind:value={config.password}
                        disabled={isRunning}
                    />
                </div>
            </div>
        </div>
    {/if}
</section>

<style>
    .card {
        padding: 1.75rem;
        border-radius: 20px;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .icon-tag {
        width: 32px;
        height: 32px;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--cyan);
    }

    .icon-tag svg {
        width: 18px;
    }

    h3 {
        margin: 0;
        font-size: 0.95rem;
        font-weight: 700;
        letter-spacing: 0.05rem;
        color: var(--text);
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
    }

    label {
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--text-dim);
        text-transform: uppercase;
        letter-spacing: 0.05rem;
    }

    .select-wrapper {
        position: relative;
    }

    input,
    select {
        width: 100%;
        background: #0c0c12;
        border: 1px solid var(--border);
        color: var(--text);
        padding: 0.85rem 1rem;
        border-radius: 12px;
        font-family: inherit;
        font-size: 0.95rem;
        transition: all 0.2s;
    }

    input:focus,
    select:focus {
        outline: none;
        border-color: var(--cyan);
        background: #11111a;
        box-shadow: 0 0 15px var(--border-glow);
    }

    input::placeholder {
        color: var(--text-vdim);
    }

    .row {
        display: flex;
        gap: 1rem;
    }
    .flex-2 {
        flex: 2;
    }

    .animate-in {
        animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    }

    @keyframes slideUp {
        from {
            opacity: 0;
            transform: translateY(10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
