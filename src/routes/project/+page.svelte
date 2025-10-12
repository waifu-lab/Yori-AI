<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { projectStore } from '$lib/stores/project';
    import { 
        launchProject, 
        stopProject, 
        getProjectStatus,
        addPluginToProject,
        removePluginFromProject,
        type ProjectData 
    } from '$lib/commands';
    
    let currentProject = $state<ProjectData | null>(null);
    let isRunning = $state(false);
    let isLoading = $state(false);
    
    // 訂閱 store
    onMount(() => {
        const unsubscribe = projectStore.subscribe(value => {
            currentProject = value;
            if (value) {
                checkProjectStatus();
            }
        });
        
        // 如果沒有當前專案，返回首頁
        if (!currentProject) {
            goto('/');
        }
        
        return unsubscribe;
    });
    
    /**
     * 檢查專案運行狀態
     */
    async function checkProjectStatus() {
        if (!currentProject) return;
        
        try {
            const status = await getProjectStatus(currentProject.name);
            isRunning = status === 'running';
        } catch (error) {
            console.error('獲取專案狀態失敗:', error);
        }
    }
    
    /**
     * 啟動/停止專案
     */
    async function toggleLauncher() {
        if (!currentProject || isLoading) return;
        
        isLoading = true;
        
        try {
            if (isRunning) {
                await stopProject(currentProject.name);
                isRunning = false;
            } else {
                await launchProject(currentProject.name);
                isRunning = true;
            }
        } catch (error) {
            console.error('切換專案狀態失敗:', error);
            alert('操作失敗: ' + error);
        } finally {
            isLoading = false;
        }
    }
    
    /**
     * 返回首頁
     */
    function backToHome() {
        projectStore.clearCurrent();
        goto('/');
    }
    
    /**
     * 前往插件頁面
     */
    function goToPlugins() {
        goto('/plugin');
    }
    
    /**
     * 獲取模式文字
     */
    function getModeText(mode: string): string {
        const modeMap: Record<string, string> = {
            'Streaming': '直播模式',
            'Desktop': '桌面模式',
            'Discord': 'Discord 模式'
        };
        return modeMap[mode] || mode;
    }
    
    /**
     * 獲取模式圖標
     */
    function getModeIcon(mode: string): string {
        const iconMap: Record<string, string> = {
            'Streaming': '📹',
            'Desktop': '🖥️',
            'Discord': '💬'
        };
        return iconMap[mode] || '📦';
    }
</script>

{#if currentProject}
<div class="container">
    <div class="header-section">
        <button class="back-btn" onclick={backToHome}>
            ← 返回
        </button>
        <div class="project-header">
            <div class="project-icon">{getModeIcon(currentProject.mode)}</div>
            <div class="project-info">
                <h1>{currentProject.name}</h1>
                <p class="project-mode">{getModeText(currentProject.mode)}</p>
                {#if currentProject.description}
                    <p class="project-desc">{currentProject.description}</p>
                {/if}
            </div>
        </div>
    </div>

    <div class="hero-section">
        <div class="status-card">
            <div class="status-indicator" class:active={isRunning}></div>
            <div class="status-info">
                <h2>系統狀態</h2>
                <p class="status-text">{isRunning ? '運行中' : '就緒'}</p>
            </div>
        </div>

        <button 
            class="launch-button" 
            onclick={toggleLauncher}
            disabled={isLoading}
        >
            <span class="button-icon">{isRunning ? '⏸️' : '▶️'}</span>
            <span>
                {#if isLoading}
                    處理中...
                {:else if isRunning}
                    停止 VTuber
                {:else}
                    啟動 VTuber
                {/if}
            </span>
        </button>
    </div>

    <div class="content-grid">
        <!-- 插件管理區 -->
        <div class="section-card">
            <div class="section-header">
                <h3>插件管理</h3>
                <button class="btn-secondary" onclick={goToPlugins}>
                    管理插件
                </button>
            </div>
            <div class="plugin-list">
                {#if currentProject.plugins.length > 0}
                    {#each currentProject.plugins as plugin}
                        <div class="plugin-item">
                            <span class="plugin-name">🔌 {plugin}</span>
                        </div>
                    {/each}
                {:else}
                    <p class="empty-text">尚未安裝任何插件</p>
                {/if}
            </div>
        </div>

        <!-- 專案資訊區 -->
        <div class="section-card">
            <h3>專案資訊</h3>
            <div class="info-list">
                <div class="info-item">
                    <span class="info-label">專案路徑</span>
                    <span class="info-value">{currentProject.path}</span>
                </div>
                <div class="info-item">
                    <span class="info-label">控制模式</span>
                    <span class="info-value">{getModeText(currentProject.mode)}</span>
                </div>
                <div class="info-item">
                    <span class="info-label">插件數量</span>
                    <span class="info-value">{currentProject.plugins.length} 個</span>
                </div>
            </div>
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
{:else}
<div class="loading-container">
    <p>載入中...</p>
</div>
{/if}

<style>
    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }

    .loading-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 50vh;
        color: #71717a;
    }

    .header-section {
        margin-bottom: 2rem;
    }

    .back-btn {
        background: #27272a;
        border: 1px solid #3f3f46;
        color: #e4e4e7;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        cursor: pointer;
        transition: all 0.2s;
        margin-bottom: 1rem;
    }

    .back-btn:hover {
        background: #3f3f46;
    }

    .project-header {
        display: flex;
        gap: 1.5rem;
        align-items: flex-start;
    }

    .project-icon {
        font-size: 4rem;
    }

    .project-info h1 {
        margin: 0 0 0.5rem;
        font-size: 2rem;
        color: #e4e4e7;
    }

    .project-mode {
        display: inline-block;
        margin: 0 0 0.5rem;
        padding: 0.25rem 0.75rem;
        background: #8b5cf6;
        color: white;
        border-radius: 0.375rem;
        font-size: 0.875rem;
        font-weight: 500;
    }

    .project-desc {
        margin: 0.5rem 0 0;
        color: #a1a1aa;
        font-size: 1rem;
    }

    .hero-section {
        background: linear-gradient(135deg, #18181b 0%, #27272a 100%);
        border-radius: 1rem;
        padding: 2.5rem;
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

    .launch-button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 10px 30px rgba(139, 92, 246, 0.3);
    }

    .launch-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .button-icon {
        font-size: 1.5rem;
    }

    .content-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .section-card {
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 1rem;
        padding: 1.5rem;
    }

    .section-card h3 {
        margin: 0 0 1rem;
        color: #e4e4e7;
        font-size: 1.25rem;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .section-header h3 {
        margin: 0;
    }

    .btn-secondary {
        padding: 0.5rem 1rem;
        background: #27272a;
        border: 1px solid #3f3f46;
        color: #e4e4e7;
        border-radius: 0.5rem;
        cursor: pointer;
        font-size: 0.875rem;
        transition: all 0.2s;
    }

    .btn-secondary:hover {
        background: #3f3f46;
        border-color: #8b5cf6;
    }

    .plugin-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .plugin-item {
        padding: 0.75rem;
        background: #0f0f0f;
        border: 1px solid #27272a;
        border-radius: 0.5rem;
    }

    .plugin-name {
        color: #e4e4e7;
        font-size: 0.95rem;
    }

    .empty-text {
        color: #71717a;
        text-align: center;
        padding: 1rem;
        font-size: 0.9rem;
    }

    .info-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .info-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem;
        background: #0f0f0f;
        border-radius: 0.5rem;
    }

    .info-label {
        color: #a1a1aa;
        font-size: 0.9rem;
    }

    .info-value {
        color: #e4e4e7;
        font-size: 0.95rem;
        font-weight: 500;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        max-width: 60%;
        text-align: right;
    }

    .stats-section {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1.5rem;
    }

    .stat-item {
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 1rem;
        padding: 2rem;
        text-align: center;
    }

    .stat-value {
        font-size: 2.5rem;
        font-weight: 700;
        color: #8b5cf6;
        margin-bottom: 0.5rem;
    }

    .stat-label {
        color: #a1a1aa;
        font-size: 0.9rem;
    }
</style>