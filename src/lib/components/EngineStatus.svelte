<script>
    let { config = {}, isRunning = false } = $props();
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
                <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                <line x1="6" y1="6" x2="6.01" y2="6"></line>
                <line x1="6" y1="18" x2="6.01" y2="18"></line>
            </svg>
        </div>
        <h3>Engine Matrix</h3>
    </div>

    <div class="field">
        <label for="local-port">Intercept Port</label>
        <div class="input-wrapper">
            <input
                id="local-port"
                type="number"
                bind:value={config.local_port}
                disabled={isRunning}
            />
            <span class="unit">TPROXY</span>
        </div>
    </div>

    <div class="matrix-info">
        <div class="info-item" class:active={isRunning}>
            <span class="dot"></span>
            <span class="text">Zero-Copy Splice Bridge</span>
        </div>
        <div class="info-item" class:active={isRunning}>
            <span class="dot"></span>
            <span class="text">DNS-over-TCP Secure Tunnel</span>
        </div>
        <div class="info-item" class:active={isRunning}>
            <span class="dot"></span>
            <span class="text">Transparent UDP Session Layer</span>
        </div>
    </div>

    {#if isRunning}
        <div class="live-stats">
            <div class="stat">
                <span class="val">FAST</span>
                <span class="lab">PIPELINE</span>
            </div>
            <div class="stat">
                <span class="val">AES-GCM</span>
                <span class="lab">CIPHER</span>
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
        color: var(--purple);
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

    .input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    input {
        width: 100%;
        background: #0c0c12;
        border: 1px solid var(--border);
        color: var(--text);
        padding: 0.85rem 1rem;
        border-radius: 12px;
        font-family: var(--font-mono);
        font-size: 0.95rem;
    }

    .unit {
        position: absolute;
        right: 1rem;
        font-size: 0.65rem;
        font-weight: 800;
        color: var(--text-vdim);
        letter-spacing: 0.05rem;
    }

    .matrix-info {
        margin-top: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
    }

    .info-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        font-size: 0.75rem;
        color: var(--text-dim);
        transition: all 0.3s;
    }

    .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: var(--text-vdim);
        transition: all 0.3s;
    }

    .info-item.active {
        color: var(--cyan);
    }

    .info-item.active .dot {
        background: var(--cyan);
        box-shadow: 0 0 10px var(--cyan);
    }

    .live-stats {
        margin-top: auto;
        padding-top: 1.5rem;
        border-top: 1px solid var(--border);
        display: flex;
        justify-content: space-between;
    }

    .stat {
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
    }

    .val {
        font-weight: 800;
        font-size: 0.8rem;
        letter-spacing: 0.1rem;
        color: var(--text);
    }

    .lab {
        font-size: 0.6rem;
        font-weight: 600;
        color: var(--text-vdim);
        text-transform: uppercase;
    }
</style>
