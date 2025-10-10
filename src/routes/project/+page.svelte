<script lang="ts">
    let status = $state('ready');
    let isRunning = $state(false);

    function toggleLauncher() {
        isRunning = !isRunning;
        status = isRunning ? 'running' : 'ready';
    }
</script>

<div class="container">
    <div class="hero-section">
        <div class="status-card">
            <div class="status-indicator" class:active={isRunning}></div>
            <div class="status-info">
                <h2>系統狀態</h2>
                <p class="status-text">{isRunning ? '運行中' : '就緒'}</p>
            </div>
        </div>

        <button class="launch-button" onclick={toggleLauncher}>
            <span class="button-icon">{isRunning ? '⏸️' : '▶️'}</span>
            <span>{isRunning ? '停止' : '啟動'} VTuber</span>
        </button>
    </div>

    <div class="features-grid">
        <div class="feature-card">
            <div class="feature-icon">🤖</div>
            <h3>AI 模型</h3>
            <p>使用先進的 AI 模型進行互動</p>
        </div>

        <div class="feature-card">
            <div class="feature-icon">🎤</div>
            <h3>語音合成</h3>
            <p>高品質的語音生成技術</p>
        </div>

        <div class="feature-card">
            <div class="feature-icon">🎨</div>
            <h3>自定義設定</h3>
            <p>完全可自定義的角色設定</p>
        </div>

        <div class="feature-card">
            <div class="feature-icon">⚡</div>
            <h3>高效能</h3>
            <p>優化的性能表現</p>
        </div>
    </div>

    <div class="stats-section">
        <div class="stat-item">
            <div class="stat-value">0</div>
            <div class="stat-label">運行時間 (分鐘)</div>
        </div>
        <div class="stat-item">
            <div class="stat-value">0</div>
            <div class="stat-label">互動次數</div>
        </div>
        <div class="stat-item">
            <div class="stat-value">100%</div>
            <div class="stat-label">系統健康度</div>
        </div>
    </div>
</div>

<style>
    .container {
        max-width: 1200px;
        margin: 0 auto;
    }

    .hero-section {
        background: linear-gradient(135deg, #18181b 0%, #27272a 100%);
        border-radius: 1rem;
        padding: 3rem;
        margin-bottom: 2rem;
        border: 1px solid #3f3f46;
    }

    .status-card {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 2rem;
    }

    .status-indicator {
        width: 16px;
        height: 16px;
        border-radius: 50%;
        background: #71717a;
        transition: all 0.3s;
    }

    .status-indicator.active {
        background: #22c55e;
        box-shadow: 0 0 20px rgba(34, 197, 94, 0.5);
        animation: pulse 2s infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }

    .status-info h2 {
        margin: 0;
        font-size: 1.25rem;
        color: #e4e4e7;
    }

    .status-text {
        margin: 0.25rem 0 0;
        color: #a1a1aa;
        font-size: 0.95rem;
    }

    .launch-button {
        width: 100%;
        padding: 1.25rem 2rem;
        font-size: 1.25rem;
        font-weight: 600;
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        color: white;
        border: none;
        border-radius: 0.75rem;
        cursor: pointer;
        transition: all 0.3s;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
    }

    .launch-button:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 30px rgba(139, 92, 246, 0.3);
    }

    .button-icon {
        font-size: 1.5rem;
    }

    .features-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .feature-card {
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 0.75rem;
        padding: 2rem;
        transition: all 0.3s;
    }

    .feature-card:hover {
        border-color: #8b5cf6;
        transform: translateY(-4px);
        box-shadow: 0 10px 30px rgba(139, 92, 246, 0.1);
    }

    .feature-icon {
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }

    .feature-card h3 {
        margin: 0 0 0.5rem;
        color: #e4e4e7;
        font-size: 1.25rem;
    }

    .feature-card p {
        margin: 0;
        color: #a1a1aa;
        font-size: 0.95rem;
    }

    .stats-section {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1.5rem;
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 0.75rem;
        padding: 2rem;
    }

    .stat-item {
        text-align: center;
    }

    .stat-value {
        font-size: 2.5rem;
        font-weight: 700;
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        margin-bottom: 0.5rem;
    }

    .stat-label {
        color: #a1a1aa;
        font-size: 0.9rem;
    }
</style>