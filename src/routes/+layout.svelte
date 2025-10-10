<script lang="ts">
    import { currentProject, type Project, updateRecentProjectName } from '$lib/stores/project';
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { page } from '$app/state';
    
    let { children } = $props();
    
    let project = $state<Project | null>(null);
    let showLayout = $derived(page.url.pathname !== '/' && !page.url.pathname.startsWith('/new'));
    let isEditingName = $state(false);
    let editedName = $state('');
    
    // 訂閱 store
    $effect(() => {
        const unsubscribe = currentProject.subscribe(value => {
            project = value;
        });
        return unsubscribe;
    });
    
    // 檢查是否需要重定向到根路徑
    onMount(async () => {
        // 載入保存的專案
        await currentProject.loadProject();
        
        // 如果沒有專案且不在根路徑或 new 頁面，導向根路徑
        if (!project && page.url.pathname !== '/' && !page.url.pathname.startsWith('/new')) {
            goto('/');
        }
    });
    
    // 當專案狀態改變時檢查
    $effect(() => {
        if (!project && page.url.pathname !== '/' && !page.url.pathname.startsWith('/new')) {
            goto('/');
        }
    });
    
    async function saveProjectName() {
        if (project && editedName.trim()) {
            const updatedProject = {
                ...project,
                name: editedName.trim()
            };
            await currentProject.setProject(updatedProject);
            
            // 更新最近專案列表中的名稱
            await updateRecentProjectName(project.path, editedName.trim());
            
            isEditingName = false;
        }
    }
</script>

<svelte:head>
    <title>AI VTuber Launcher</title>
</svelte:head>

{#if showLayout}
    <div class="app">
        <aside class="sidebar">
            <div class="project-header">
                {#if isEditingName}
                    <div class="edit-name-container">
                        <input 
                            type="text" 
                            bind:value={editedName}
                            onkeydown={(e) => {
                                if (e.key === 'Enter') saveProjectName();
                                else if (e.key === 'Escape') {
                                    isEditingName = false;
                                    editedName = '';
                                }
                            }}
                            class="name-input"
                            autofocus
                        />
                        <div class="edit-actions">
                            <button class="save-btn" onclick={saveProjectName} title="儲存">✓</button>
                            <button class="cancel-btn" onclick={() => {
                                isEditingName = false;
                                editedName = '';
                            }} title="取消">✕</button>
                        </div>
                    </div>
                {:else}
                    <div class="name-display">
                        <h2 class="project-name">{project!.name}</h2>
                        <button class="edit-btn" onclick={() => {
                            if (project) {
                                editedName = project.name;
                                isEditingName = true;
                            }
                        }} title="編輯名稱">✎</button>
                    </div>
                {/if}
            </div>
            
            {#if project}
                <nav class="plugin-list">
                    <div class="section-title">已安裝插件</div>
                    {#if project.plugins.length > 0}
                        {#each project.plugins as plugin}
                            <a href="/plugins/{plugin}" class="plugin-item">
                                <span class="icon">🧩</span>
                                <span>{plugin}</span>
                            </a>
                        {/each}
                    {:else}
                        <div class="empty-state">尚未安裝插件</div>
                    {/if}
                </nav>
            {/if}
        </aside>

        <main class="main-content">
            {@render children()}
        </main>
    </div>
{:else}
    <div class="fullscreen">
        {@render children()}
    </div>
{/if}

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        background: #0f0f0f;
        color: #e4e4e7;
    }

    .app {
        display: flex;
        height: 100vh;
        overflow: hidden;
    }

    .sidebar {
        width: 240px;
        background: linear-gradient(180deg, #18181b 0%, #09090b 100%);
        border-right: 1px solid #27272a;
        display: flex;
        flex-direction: column;
        padding: 1.5rem 0;
    }

    .project-header {
        padding: 0 1.5rem 1.5rem;
        border-bottom: 1px solid #27272a;
        margin-bottom: 1rem;
    }

    .name-display {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .project-name {
        margin: 0;
        font-size: 1.5rem;
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        flex: 1;
    }

    .edit-btn {
        width: 32px;
        height: 32px;
        background: transparent;
        border: 1px solid #3f3f46;
        border-radius: 0.375rem;
        color: #a1a1aa;
        font-size: 1rem;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    .edit-btn:hover {
        background: #27272a;
        border-color: #8b5cf6;
        color: #8b5cf6;
    }

    .edit-name-container {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .name-input {
        width: calc(100% - 2rem);
        padding: 0.5rem 0.75rem;
        background: #0f0f0f;
        border: 1px solid #8b5cf6;
        border-radius: 0.375rem;
        color: #e4e4e7;
        font-size: 1.25rem;
        font-weight: 600;
        font-family: inherit;
        box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.1);
    }

    .name-input:focus {
        outline: none;
        border-color: #8b5cf6;
    }

    .edit-actions {
        display: flex;
        gap: 0.5rem;
    }

    .save-btn,
    .cancel-btn {
        flex: 1;
        padding: 0.5rem;
        border-radius: 0.375rem;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
    }

    .save-btn {
        background: #22c55e;
        color: white;
    }

    .save-btn:hover {
        background: #16a34a;
    }

    .cancel-btn {
        background: #27272a;
        border: 1px solid #3f3f46;
        color: #a1a1aa;
    }

    .cancel-btn:hover {
        background: #3f3f46;
        color: #e4e4e7;
    }

    .section-title {
        padding: 0.5rem 1.5rem;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: #71717a;
    }

    .plugin-list {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        padding: 0 0.75rem;
    }

    .plugin-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem 1rem;
        border-radius: 0.5rem;
        text-decoration: none;
        color: #a1a1aa;
        transition: all 0.2s;
        font-size: 0.95rem;
    }

    .plugin-item:hover {
        background: #27272a;
        color: #e4e4e7;
    }

    .plugin-item .icon {
        font-size: 1.25rem;
    }

    .empty-state {
        padding: 1rem 1.5rem;
        text-align: center;
        color: #71717a;
        font-size: 0.9rem;
    }

    .main-content {
        flex: 1;
        overflow-y: auto;
        padding: 2rem;
        background: #0f0f0f;
    }

    /* 全屏模式（home 和 new 頁面） */
    .fullscreen {
        width: 100vw;
        height: 100vh;
        overflow-y: auto;
        background: #0f0f0f;
    }

    /* 自定義滾動條 */
    .main-content::-webkit-scrollbar,
    .fullscreen::-webkit-scrollbar {
        width: 8px;
    }

    .main-content::-webkit-scrollbar-track,
    .fullscreen::-webkit-scrollbar-track {
        background: #18181b;
    }

    .main-content::-webkit-scrollbar-thumb,
    .fullscreen::-webkit-scrollbar-thumb {
        background: #3f3f46;
        border-radius: 4px;
    }

    .main-content::-webkit-scrollbar-thumb:hover,
    .fullscreen::-webkit-scrollbar-thumb:hover {
        background: #52525b;
    }
</style>