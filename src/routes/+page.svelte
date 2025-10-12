<script lang="ts">
    import { goto } from '$app/navigation';
    import { projectStore } from '$lib/stores/project';
    import { getRecentProjects, removeProject } from '$lib/commands';
    import type { ProjectData } from '$lib/commands';
    import { onMount } from 'svelte';
    import { open } from '@tauri-apps/plugin-dialog';
    
    // 最近的專案列表
    let recentProjects = $state<ProjectData[]>([]);
    let loading = $state(true);
    
    onMount(async () => {
        try {
            recentProjects = await getRecentProjects();
        } catch (error) {
            console.error('載入最近專案失敗:', error);
        } finally {
            loading = false;
        }
    });
    
    /**
     * 載入選中的專案
     */
    async function loadProject(project: ProjectData) {
        projectStore.setCurrent(project);
        goto('/project');
    }
    
    /**
     * 創建新專案
     */
    function createNewProject() {
        goto('/new');
    }
    
    /**
     * 從文件系統載入專案
     */
    async function openProjectFolder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: '選擇專案資料夾'
            });
            
            if (selected) {
                // TODO: 從資料夾載入專案配置
                alert(`選擇的資料夾: ${selected}`);
            }
        } catch (error) {
            console.error('選擇資料夾失敗:', error);
        }
    }
    
    /**
     * 從列表中刪除專案
     */
    async function deleteProject(event: MouseEvent, project: ProjectData) {
        event.stopPropagation();
        
        if (confirm(`確定要刪除專案「${project.name}」嗎？`)) {
            try {
                const success = await removeProject(project.name);
                if (success) {
                    // 重新載入專案列表
                    recentProjects = await getRecentProjects();
                }
            } catch (error) {
                console.error('刪除專案失敗:', error);
                alert('刪除專案失敗');
            }
        }
    }
    
    /**
     * 格式化日期顯示
     */
    function formatDate(timestamp: number): string {
        const date = new Date(timestamp);
        return date.toLocaleDateString('zh-TW', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit'
        });
    }
    
    /**
     * 獲取模式的顯示文字
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
     * 獲取模式的圖標
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

<div class="home-container">
    <div class="welcome-section">
        <h1>歡迎使用 AI VTuber Launcher</h1>
        <p>選擇一個專案開始，或創建新專案</p>
    </div>

    <div class="actions">
        <button class="action-card create" onclick={createNewProject}>
            <div class="icon">➕</div>
            <h3>創建新專案</h3>
            <p>從頭開始設定你的 AI VTuber</p>
        </button>

        <button class="action-card load" onclick={openProjectFolder}>
            <div class="icon">📁</div>
            <h3>載入專案</h3>
            <p>從本地載入現有專案</p>
        </button>
    </div>

    {#if loading}
        <div class="loading">
            <p>載入中...</p>
        </div>
    {:else if recentProjects.length > 0}
        <div class="recent-section">
            <h2>最近的專案</h2>
            <div class="recent-list">
                {#each recentProjects as project}
                    <div class="recent-item-wrapper">
                        <button 
                            class="recent-item"
                            onclick={() => loadProject(project)}
                        >
                            <div class="recent-icon">{getModeIcon(project.mode)}</div>
                            <div class="recent-info">
                                <div class="recent-header">
                                    <div class="recent-name">{project.name}</div>
                                    <div class="recent-mode">{getModeText(project.mode)}</div>
                                </div>
                                <div class="recent-path">{project.path}</div>
                                <div class="recent-meta">
                                    <span>上次開啟: {formatDate(project.last_opened)}</span>
                                    {#if project.plugins.length > 0}
                                        <span>• {project.plugins.length} 個插件</span>
                                    {/if}
                                </div>
                            </div>
                        </button>
                        <button 
                            class="delete-btn"
                            onclick={(e) => deleteProject(e, project)}
                            title="刪除專案"
                        >
                            ✕
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {:else}
        <div class="empty-state">
            <p>還沒有任何專案</p>
            <p class="hint">點擊上方按鈕開始創建或載入專案</p>
        </div>
    {/if}
</div>

<style>
    .home-container {
        max-width: 900px;
        margin: 0 auto;
        padding: 2rem;
    }

    .welcome-section {
        text-align: center;
        margin-bottom: 3rem;
    }

    .welcome-section h1 {
        font-size: 2.5rem;
        margin: 0 0 1rem;
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .welcome-section p {
        font-size: 1.1rem;
        color: #a1a1aa;
        margin: 0;
    }

    .actions {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 1.5rem;
        margin-bottom: 3rem;
    }

    .action-card {
        background: linear-gradient(135deg, #18181b 0%, #27272a 100%);
        border: 2px solid #3f3f46;
        border-radius: 1rem;
        padding: 2.5rem;
        text-align: center;
        cursor: pointer;
        transition: all 0.3s;
        width: 100%;
    }

    .action-card:hover {
        border-color: #8b5cf6;
        transform: translateY(-4px);
        box-shadow: 0 10px 30px rgba(139, 92, 246, 0.2);
    }

    .action-card.create {
        border-color: #8b5cf6;
    }

    .action-card .icon {
        font-size: 3rem;
        margin-bottom: 1rem;
    }

    .action-card h3 {
        margin: 0 0 0.5rem;
        font-size: 1.5rem;
        color: #e4e4e7;
    }

    .action-card p {
        margin: 0;
        color: #a1a1aa;
        font-size: 0.95rem;
    }

    .recent-section {
        margin-top: 3rem;
    }

    .recent-section h2 {
        font-size: 1.5rem;
        margin: 0 0 1rem;
        color: #e4e4e7;
    }

    .recent-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .recent-item-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .recent-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 0.75rem;
        padding: 1.25rem;
        cursor: pointer;
        transition: all 0.2s;
        flex: 1;
        text-align: left;
    }

    .recent-item:hover {
        border-color: #8b5cf6;
        background: #1f1f23;
    }

    .delete-btn {
        width: 40px;
        height: 40px;
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 0.5rem;
        color: #71717a;
        font-size: 1.25rem;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .delete-btn:hover {
        background: #ef4444;
        border-color: #ef4444;
        color: white;
    }

    .recent-icon {
        font-size: 2.5rem;
        flex-shrink: 0;
    }

    .recent-info {
        flex: 1;
        min-width: 0;
    }

    .recent-header {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-bottom: 0.5rem;
    }

    .recent-name {
        font-size: 1.1rem;
        font-weight: 600;
        color: #e4e4e7;
    }

    .recent-mode {
        font-size: 0.75rem;
        padding: 0.25rem 0.5rem;
        background: #8b5cf6;
        color: white;
        border-radius: 0.375rem;
        font-weight: 500;
    }

    .recent-path {
        font-size: 0.875rem;
        color: #71717a;
        margin-bottom: 0.5rem;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .recent-meta {
        font-size: 0.75rem;
        color: #52525b;
        display: flex;
        gap: 0.5rem;
    }

    .loading,
    .empty-state {
        text-align: center;
        padding: 3rem;
        color: #71717a;
    }

    .empty-state .hint {
        margin-top: 0.5rem;
        font-size: 0.9rem;
        color: #52525b;
    }
</style>
