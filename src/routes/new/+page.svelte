<script lang="ts">
    import { goto } from '$app/navigation';
    import { currentProject } from '$lib/stores/project';
    import { open } from '@tauri-apps/plugin-dialog';
    
    let projectName = $state('');
    let projectPath = $state('');
    let description = $state('');
    let errors = $state<Record<string, string>>({});
    
    function validateForm() {
        errors = {};
        
        if (!projectName.trim()) {
            errors.name = '請輸入專案名稱';
        }
        
        if (!projectPath.trim()) {
            errors.path = '請輸入專案路徑';
        }
        
        return Object.keys(errors).length === 0;
    }
    
    function createProject() {
        if (!validateForm()) {
            return;
        }
        
        const newProject = {
            name: projectName.trim(),
            path: projectPath.trim(),
            plugins: []
        };
        
        // 設定當前專案
        currentProject.setProject(newProject);
        
        // 儲存到最近專案列表
        const stored = localStorage.getItem('recentProjects');
        const recentProjects = stored ? JSON.parse(stored) : [];
        
        // 避免重複，先移除相同路徑的專案
        const filtered = recentProjects.filter((p: any) => p.path !== newProject.path);
        
        // 加入新專案到最前面
        filtered.unshift({
            name: newProject.name,
            path: newProject.path
        });
        
        // 最多保留 10 個最近專案
        const limited = filtered.slice(0, 10);
        localStorage.setItem('recentProjects', JSON.stringify(limited));
        
        // 導向首頁
        goto('/');
    }
    
    function cancel() {
        goto('/home');
    }
    
    async function browsePath() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: '選擇專案資料夾'
            });
            
            if (selected && typeof selected === 'string') {
                projectPath = selected;
                
                // 如果專案名稱還沒設定，自動填入資料夾名稱
                if (!projectName) {
                    const folderName = selected.split(/[/\\]/).pop() || '';
                    projectName = folderName;
                }
            }
        } catch (error) {
            console.error('選擇資料夾時發生錯誤:', error);
        }
    }
</script>

<div class="new-container">
    <div class="header">
        <h1>創建新專案</h1>
        <p>設定你的 AI VTuber 專案</p>
    </div>

    <form class="form" onsubmit={(e) => { e.preventDefault(); createProject(); }}>
        <div class="form-group">
            <label for="name">專案名稱 *</label>
            <input 
                id="name"
                type="text" 
                bind:value={projectName}
                placeholder="例如：我的第一個 VTuber"
                class:error={errors.name}
            />
            {#if errors.name}
                <span class="error-message">{errors.name}</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="path">專案路徑 *</label>
            <div class="path-input">
                <input 
                    id="path"
                    type="text" 
                    bind:value={projectPath}
                    placeholder="C:/Projects/my-vtuber"
                    class:error={errors.path}
                />
                <button type="button" class="browse-btn" onclick={browsePath}>
                    瀏覽...
                </button>
            </div>
            {#if errors.path}
                <span class="error-message">{errors.path}</span>
            {/if}
        </div>

        <div class="form-group">
            <label for="description">專案描述（可選）</label>
            <textarea 
                id="description"
                bind:value={description}
                placeholder="簡單描述這個專案的用途..."
                rows="4"
            ></textarea>
        </div>

        <div class="info-card">
            <div class="info-icon">ℹ️</div>
            <div class="info-content">
                <strong>提示：</strong>
                專案路徑將用於儲存設定檔和相關資源。請確保該路徑有寫入權限。
            </div>
        </div>

        <div class="form-actions">
            <button type="button" class="btn-cancel" onclick={cancel}>
                取消
            </button>
            <button type="submit" class="btn-create">
                創建專案
            </button>
        </div>
    </form>
</div>

<style>
    .new-container {
        max-width: 600px;
        margin: 0 auto;
        padding: 2rem;
    }

    .header {
        text-align: center;
        margin-bottom: 3rem;
    }

    .header h1 {
        font-size: 2.5rem;
        margin: 0 0 0.5rem;
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .header p {
        font-size: 1.1rem;
        color: #a1a1aa;
        margin: 0;
    }

    .form {
        background: #18181b;
        border: 1px solid #27272a;
        border-radius: 1rem;
        padding: 2rem;
    }

    .form-group {
        margin-bottom: 1.5rem;
    }

    .form-group:last-of-type {
        margin-bottom: 2rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        color: #e4e4e7;
        font-weight: 500;
        font-size: 0.95rem;
    }

    input[type="text"],
    textarea {
        width: calc(100% - 2rem);
        padding: 0.75rem 1rem;
        background: #0f0f0f;
        border: 1px solid #3f3f46;
        border-radius: 0.5rem;
        color: #e4e4e7;
        font-size: 1rem;
        transition: all 0.2s;
        font-family: inherit;
    }

    input[type="text"]:focus,
    textarea:focus {
        outline: none;
        border-color: #8b5cf6;
        box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.1);
    }

    input.error,
    textarea.error {
        border-color: #ef4444;
    }

    .error-message {
        display: block;
        margin-top: 0.5rem;
        color: #ef4444;
        font-size: 0.85rem;
    }

    .path-input {
        display: flex;
        gap: 0.5rem;
    }

    .path-input input {
        flex: 1;
    }

    .browse-btn {
        padding: 0.75rem 1.5rem;
        background: #27272a;
        border: 1px solid #3f3f46;
        border-radius: 0.5rem;
        color: #e4e4e7;
        font-size: 0.95rem;
        cursor: pointer;
        transition: all 0.2s;
        white-space: nowrap;
    }

    .browse-btn:hover {
        background: #3f3f46;
        border-color: #52525b;
    }

    textarea {
        resize: vertical;
        min-height: 100px;
    }

    .info-card {
        display: flex;
        gap: 1rem;
        padding: 1rem;
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
        border-radius: 0.5rem;
        margin-bottom: 2rem;
    }

    .info-icon {
        font-size: 1.5rem;
        flex-shrink: 0;
    }

    .info-content {
        color: #a1a1aa;
        font-size: 0.9rem;
        line-height: 1.5;
    }

    .info-content strong {
        color: #e4e4e7;
    }

    .form-actions {
        display: flex;
        gap: 1rem;
        justify-content: flex-end;
    }

    .btn-cancel,
    .btn-create {
        padding: 0.75rem 2rem;
        border-radius: 0.5rem;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
    }

    .btn-cancel {
        background: transparent;
        border: 1px solid #3f3f46;
        color: #a1a1aa;
    }

    .btn-cancel:hover {
        background: #27272a;
        color: #e4e4e7;
    }

    .btn-create {
        background: linear-gradient(135deg, #8b5cf6 0%, #ec4899 100%);
        color: white;
    }

    .btn-create:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 30px rgba(139, 92, 246, 0.3);
    }
</style>
