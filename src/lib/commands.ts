import { invoke } from '@tauri-apps/api/core';

export type ProjectControlMode = 'Streaming' | 'Desktop' | 'Discord';

export interface ProjectData {
	id: string;
	name: string;
	path: string;
	description: string;
	mode: ProjectControlMode;
	plugins: string[];
	created_at: number;
	last_opened: number;
	metadata: ProjectMetadata;
}

export interface ProjectMetadata {
	plugin_data: Record<string, [string, string]>;
}

export interface PluginData {
	name: string;
	version: string;
	author: string;
	description: string;
	is_deprecated: boolean;
	need_upgrade: boolean;
}

/**
 * 建立新專案
 * @param name 專案名稱
 * @param path 專案路徑
 * @param description 專案描述
 * @param mode 專案模式, 'Streaming' | 'Desktop' | 'Discord'
 */
export async function createProject(
	name: string,
	path: string,
	description: string,
	mode: ProjectControlMode
): Promise<string> {
	return await invoke<string>('create_project', { name, path, description, mode });
}

/**
 * 刪除專案
 * @param id 專案id
 */
export async function deleteProject(id: string): Promise<boolean> {
	return await invoke<boolean>('delete_project', { id });
}

/**
 * 儲存專案
 * @param project 專案資料
 */
export async function saveProject(project: ProjectData): Promise<void> {
	return await invoke<void>('save_project', { project });
}

/**
 * 重新載入所有專案
 */
export async function reloadProjects(): Promise<void> {
	return await invoke<void>('reload_projects');
}

/**
 * 取得最近的專案列表
 */
export async function listRecentProjects(): Promise<ProjectData[]> {
	return await invoke<ProjectData[]>('list_recent_projects');
}

/**
 * 啟動專案
 * @param id 專案id
 */
export async function launchProject(id: string): Promise<void> {
	return await invoke<void>('launch_project', { id });
}

/**
 * 新增插件
 * @param filePath 插件檔案路徑
 */
export async function addPlugin(filePath: string): Promise<void> {
	return await invoke<void>('add_plugin', { filePath });
}

/**
 * 移除插件
 * @param pluginName 插件名稱
 */
export async function removePlugin(pluginName: string): Promise<void> {
	return await invoke<void>('remove_plugin', { pluginName });
}

/**
 * 列出所有插件
 */
export async function listPlugins(): Promise<PluginData[]> {
	return await invoke<PluginData[]>('list_plugins');
}
