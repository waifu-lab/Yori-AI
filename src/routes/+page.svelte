<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { open } from '@tauri-apps/plugin-dialog';
	import {
		listRecentProjects,
		createProject,
		reloadProjects,
		type ProjectData,
		type ProjectControlMode
	} from '$lib/commands';
	import { projects } from '$lib/stores/project';
	import ProjectCard from '$lib/components/ProjectCard.svelte';

	let recentProjects = $state<ProjectData[]>([]);
	let loading = $state(true);
	let showCreateDialog = $state(false);
	let creating = $state(false);

	let projectName = $state('');
	let projectPath = $state('');
	let projectDescription = $state('');
	let projectMode = $state<ProjectControlMode>('Streaming');

	onMount(async () => {
		await loadProjects();
	});

	async function loadProjects() {
		try {
			loading = true;
			const projectList = await listRecentProjects();
			recentProjects = projectList;
			projects.set(projectList);
		} catch (error) {
			console.error('載入專案失敗:', error);
		} finally {
			loading = false;
		}
	}

	function handleProjectClick(project: ProjectData) {
		goto(`/project?id=${project.id}`);
	}

	function handleCreateProject() {
		showCreateDialog = true;
		projectName = '';
		projectPath = '';
		projectDescription = '';
		projectMode = 'Streaming';
	}

	function closeDialog() {
		showCreateDialog = false;
	}

	async function selectFolder() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: '選擇專案資料夾'
			});

			if (selected && typeof selected === 'string') {
				projectPath = selected;
			}
		} catch (error) {
			console.error('選擇資料夾失敗:', error);
		}
	}

	async function handleSubmit() {
		if (!projectName.trim() || !projectPath.trim()) {
			return;
		}

		try {
			creating = true;
			await createProject(projectName, projectPath, projectDescription, projectMode);
			await reloadProjects();
			await loadProjects();
			closeDialog();
		} catch (error) {
			console.error('建立專案失敗:', error);
			alert('建立專案失敗: ' + error);
		} finally {
			creating = false;
		}
	}
</script>

<div class="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-10 space-y-1">
		<h1 class="text-3xl font-semibold tracking-tight text-white">專案</h1>
		<p class="text-muted-foreground text-sm">管理您的 VTuber 專案</p>
	</div>

	<button
		onclick={handleCreateProject}
		class="bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-ring mb-8 inline-flex h-10 items-center justify-center gap-2 rounded-md px-4 py-2 text-sm font-medium shadow transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
	>
		<span class="icon-[lucide--plus]"></span>
		建立新專案
	</button>

	{#if loading}
		<div class="flex h-[450px] items-center justify-center">
			<div class="border-muted-foreground h-8 w-8 animate-spin rounded-full border-2 border-t-transparent"></div>
		</div>
	{/if}

	{#if !loading && recentProjects.length === 0}
		<div
			class="border-border bg-muted/10 flex h-[450px] flex-col items-center justify-center space-y-4 rounded-lg border border-dashed p-8"
		>
			<div class="icon-[lucide--folder-open] text-muted-foreground h-16 w-16"></div>
			<div class="space-y-2 text-center">
				<h3 class="text-lg font-medium">尚無專案</h3>
				<p class="text-muted-foreground text-sm">開始建立您的第一個 VTuber 專案</p>
			</div>
		</div>
	{/if}

	{#if !loading && recentProjects.length > 0}
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each recentProjects as project}
				<ProjectCard {project} onclick={() => handleProjectClick(project)} />
			{/each}
		</div>
	{/if}
</div>

<!-- Create Project Dialog -->
{#if showCreateDialog}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/80"
		onclick={closeDialog}
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="border-border bg-background data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] fixed top-[50%] left-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border p-6 shadow-lg duration-200 sm:rounded-lg"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex flex-col space-y-1.5 text-center sm:text-left">
				<h2 class="text-lg leading-none font-semibold tracking-tight">建立新專案</h2>
				<p class="text-muted-foreground text-sm">填寫專案資訊以開始</p>
			</div>
			<button
				onclick={closeDialog}
				class="ring-offset-background focus:ring-ring absolute top-4 right-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:pointer-events-none"
				disabled={creating}
			>
				<span class="icon-[lucide--x] h-4 w-4"></span>
				<span class="sr-only">關閉</span>
			</button>

			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
				class="grid gap-4 py-4"
			>
				<div class="grid gap-2">
					<label
						for="projectName"
						class="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						專案名稱 <span class="text-destructive">*</span>
					</label>
					<input
						id="projectName"
						type="text"
						bind:value={projectName}
						placeholder="我的 VTuber 專案"
						required
						disabled={creating}
						class="border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:ring-1 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
					/>
				</div>

				<div class="grid gap-2">
					<label
						for="projectPath"
						class="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						專案路徑 <span class="text-destructive">*</span>
					</label>
					<div class="flex gap-2">
						<input
							id="projectPath"
							type="text"
							bind:value={projectPath}
							placeholder="選擇專案資料夾"
							required
							readonly
							disabled={creating}
							class="border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:ring-1 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
						/>
						<button
							type="button"
							onclick={selectFolder}
							disabled={creating}
							class="border-input bg-background hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring inline-flex h-9 items-center justify-center gap-2 rounded-md border px-3 text-sm font-medium whitespace-nowrap shadow-sm transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
						>
							<span class="icon-[lucide--folder-open] h-4 w-4"></span>
							瀏覽
						</button>
					</div>
				</div>

				<div class="grid gap-2">
					<label
						for="projectDescription"
						class="text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
						>專案描述</label
					>
					<textarea
						id="projectDescription"
						bind:value={projectDescription}
						placeholder="描述這個專案的用途..."
						rows="3"
						disabled={creating}
						class="border-input placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[60px] w-full rounded-md border bg-transparent px-3 py-2 text-sm shadow-sm focus-visible:ring-1 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
					></textarea>
				</div>

				<div class="grid gap-2">
					<span class="text-sm leading-none font-medium">
						專案模式 <span class="text-destructive">*</span>
					</span>
					<div class="grid grid-cols-3 gap-2">
						<button
							type="button"
							onclick={() => (projectMode = 'Streaming')}
							disabled={creating}
							class="border-input hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring inline-flex h-auto flex-col items-center justify-center gap-2 rounded-md border-2 bg-transparent p-3 text-sm font-medium transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 {projectMode ===
							'Streaming'
								? 'border-primary bg-primary/10'
								: ''}"
						>
							<span class="icon-[lucide--video] h-5 w-5"></span>
							<span class="text-xs">直播</span>
						</button>
						<button
							type="button"
							onclick={() => (projectMode = 'Desktop')}
							disabled={creating}
							class="border-input hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring inline-flex h-auto flex-col items-center justify-center gap-2 rounded-md border-2 bg-transparent p-3 text-sm font-medium transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 {projectMode ===
							'Desktop'
								? 'border-primary bg-primary/10'
								: ''}"
						>
							<span class="icon-[lucide--monitor] h-5 w-5"></span>
							<span class="text-xs">桌面</span>
						</button>
						<button
							type="button"
							onclick={() => (projectMode = 'Discord')}
							disabled={creating}
							class="border-input hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring inline-flex h-auto flex-col items-center justify-center gap-2 rounded-md border-2 bg-transparent p-3 text-sm font-medium transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 {projectMode ===
							'Discord'
								? 'border-primary bg-primary/10'
								: ''}"
						>
							<span class="icon-[lucide--message-circle] h-5 w-5"></span>
							<span class="text-xs">Discord</span>
						</button>
					</div>
				</div>

				<div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
					<button
						type="button"
						onclick={closeDialog}
						disabled={creating}
						class="border-input bg-background hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring inline-flex h-9 items-center justify-center gap-2 rounded-md border px-4 py-2 text-sm font-medium shadow-sm transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
					>
						取消
					</button>
					<button
						type="submit"
						disabled={creating || !projectName.trim() || !projectPath.trim()}
						class="bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-ring inline-flex h-9 items-center justify-center gap-2 rounded-md px-4 py-2 text-sm font-medium shadow transition-colors focus-visible:ring-1 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
					>
						{#if creating}
							<span class="border-primary-foreground h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"
							></span>
							建立中...
						{:else}
							建立專案
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
