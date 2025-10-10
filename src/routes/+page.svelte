<script lang="ts">
    import { goto } from '$app/navigation';
    import { currentProject, getRecentProjects, removeRecentProject } from '$lib/stores/project';
    import { onMount } from 'svelte';
    
    // 從 Tauri store 讀取最近的專案列表
    let recentProjects = $state<Array<{ name: string; path: string }>>([]);
    
    onMount(async () => {
        // 載入最近專案列表
        recentProjects = await getRecentProjects();
    });
    
    async function loadProject(name: string, path: string) {
        await currentProject.setProject({
            name,
            path,
            plugins: [] // 可以從 Tauri store 載入插件列表
        });
        goto('/project');
    }
    
    function createNewProject() {
        goto('/new');
    }
    
    function openFileDialog() {
        // 使用 HTML5 file input 或其他方式
        alert('載入專案功能：可以使用資料夾選擇器或直接輸入專案路徑');
    }
    
    async function deleteProject(event: Event, path: string) {
        event.stopPropagation(); // 防止觸發載入專案
        
        if (confirm('確定要從列表中移除此專案嗎？')) {
            recentProjects = await removeRecentProject(path);
        }
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

        <button class="action-card load" onclick={openFileDialog}>
            <div class="icon">📁</div>
            <h3>載入專案</h3>
            <p>從本地載入現有專案</p>
        </button>
    </div>

    {#if recentProjects.length > 0}
        <div class="recent-section">
            <h2>最近的專案</h2>
            <div class="recent-list">
                {#each recentProjects as project}
                    <div class="recent-item-wrapper">
                        <button 
                            class="recent-item"
                            onclick={() => loadProject(project.name, project.path)}
                        >
                            <div class="recent-icon">📦</div>
                            <div class="recent-info">
                                <div class="recent-name">{project.name}</div>
                                <div class="recent-path">{project.path}</div>
                            </div>
                        </button>
                        <button 
                            class="delete-btn"
                            onclick={(e) => deleteProject(e, project.path)}
                            title="從列表移除"
                        >
                            ✕
                        </button>
                    </div>
                {/each}
            </div>
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
        font-size: 2rem;
    }

    .recent-info {
        flex: 1;
    }

    .recent-name {
        font-size: 1.1rem;
        font-weight: 600;
        color: #e4e4e7;
        margin-bottom: 0.25rem;
    }

    .recent-path {
        font-size: 0.85rem;
        color: #71717a;
    }
</style>
