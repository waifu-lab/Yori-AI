/**
 * Tauri Commands Interface
 * 
 * 前端與後端的 Tauri 命令接口
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * 專案控制模式
 */
export enum ProjectControlMode {
    Streaming = 'Streaming',
    Desktop = 'Desktop',
    Discord = 'Discord'
}

/**
 * 專案資料結構（匹配後端 ProjectData）
 */
export interface ProjectData {
    name: string;
    path: string;
    description: string;
    mode: ProjectControlMode;
    plugins: string[];
    created_at: number; // SystemTime as milliseconds
    last_opened: number; // SystemTime as milliseconds
}

/**
 * 創建新專案
 */
export async function createProject(
    name: string,
    path: string,
    description: string,
    mode: ProjectControlMode
): Promise<void> {
    await invoke('create_project', { name, path, description, mode });
}

/**
 * 獲取最近的專案列表
 */
export async function getRecentProjects(): Promise<ProjectData[]> {
    return await invoke<ProjectData[]>('get_recent_projects');
}

/**
 * 通過名稱獲取專案
 */
export async function getProjectByName(name: string): Promise<ProjectData | null> {
    return await invoke<ProjectData | null>('get_project_by_name', { name });
}

/**
 * 移除專案
 */
export async function removeProject(name: string): Promise<boolean> {
    return await invoke<boolean>('remove_project', { name });
}

/**
 * 更新專案名稱
 */
export async function updateProjectName(oldName: string, newName: string): Promise<void> {
    await invoke('update_project_name', { oldName, newName });
}

/**
 * 更新專案描述
 */
export async function updateProjectDescription(name: string, description: string): Promise<void> {
    await invoke('update_project_description', { name, description });
}

/**
 * 添加插件到專案
 */
export async function addPluginToProject(projectName: string, pluginId: string): Promise<void> {
    await invoke('add_plugin_to_project', { projectName, pluginId });
}

/**
 * 從專案移除插件
 */
export async function removePluginFromProject(projectName: string, pluginId: string): Promise<void> {
    await invoke('remove_plugin_from_project', { projectName, pluginId });
}

/**
 * 啟動專案
 */
export async function launchProject(name: string): Promise<void> {
    await invoke('launch_project', { name });
}

/**
 * 停止專案
 */
export async function stopProject(name: string): Promise<void> {
    await invoke('stop_project', { name });
}

/**
 * 獲取專案運行狀態
 */
export async function getProjectStatus(name: string): Promise<'running' | 'stopped'> {
    return await invoke<'running' | 'stopped'>('get_project_status', { name });
}
