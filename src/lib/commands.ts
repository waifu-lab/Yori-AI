/**
 * Tauri Commands Interface
 * 
 * 這個檔案定義了前端需要的所有 Tauri commands
 * 後端需要實作這些 commands
 */

import type { Project } from './stores/project';
import { invoke } from '@tauri-apps/api/core';

/**
 * 儲存當前專案
 * @param project - 專案資料
 */
export async function saveCurrentProject(project: Project): Promise<void> {
    await invoke('save_current_project', { project });
}

/**
 * 清除當前專案
 */
export async function clearCurrentProject(): Promise<void> {
    await invoke('clear_current_project');
}

/**
 * 載入當前專案
 * @returns 當前專案或 null
 */
export async function loadCurrentProject(): Promise<Project | null> {
    return await invoke<Project | null>('load_current_project');
}

/**
 * 獲取最近的專案列表
 * @returns 最近專案列表
 */
export async function getRecentProjects(): Promise<Array<{ name: string; path: string }>> {
    return await invoke<Array<{ name: string; path: string }>>('get_recent_projects');
}

/**
 * 新增專案到最近列表
 * @param project - 專案資料（名稱和路徑）
 * @returns 更新後的最近專案列表
 */
export async function addRecentProject(project: { name: string; path: string }): Promise<Array<{ name: string; path: string }>> {
    return await invoke<Array<{ name: string; path: string }>>('add_recent_project', { project });
}

/**
 * 從最近列表移除專案
 * @param path - 專案路徑
 * @returns 更新後的最近專案列表
 */
export async function removeRecentProject(path: string): Promise<Array<{ name: string; path: string }>> {
    return await invoke<Array<{ name: string; path: string }>>('remove_recent_project', { path });
}

/**
 * 更新最近專案的名稱
 * @param path - 專案路徑
 * @param newName - 新名稱
 * @returns 更新後的最近專案列表
 */
export async function updateRecentProjectName(path: string, newName: string): Promise<Array<{ name: string; path: string }>> {
    return await invoke<Array<{ name: string; path: string }>>('update_recent_project_name', { path, newName });
}

/*
 * ===== 後端需要實作的 Rust Commands =====
 * 
 * 在 src-tauri/src/lib.rs 或相關檔案中需要實作以下 commands：
 * 
 * #[tauri::command]
 * async fn save_current_project(project: Project) -> Result<(), String> {
 *     // 儲存專案到檔案系統或資料庫
 * }
 * 
 * #[tauri::command]
 * async fn clear_current_project() -> Result<(), String> {
 *     // 清除當前專案
 * }
 * 
 * #[tauri::command]
 * async fn load_current_project() -> Result<Option<Project>, String> {
 *     // 載入當前專案
 * }
 * 
 * #[tauri::command]
 * async fn get_recent_projects() -> Result<Vec<RecentProject>, String> {
 *     // 取得最近專案列表
 * }
 * 
 * #[tauri::command]
 * async fn add_recent_project(project: RecentProject) -> Result<Vec<RecentProject>, String> {
 *     // 新增到最近專案列表
 *     // 需處理：去重、限制數量（最多10個）
 * }
 * 
 * #[tauri::command]
 * async fn remove_recent_project(path: String) -> Result<Vec<RecentProject>, String> {
 *     // 從最近列表移除
 * }
 * 
 * #[tauri::command]
 * async fn update_recent_project_name(path: String, new_name: String) -> Result<Vec<RecentProject>, String> {
 *     // 更新最近專案的名稱
 * }
 * 
 * 資料結構建議：
 * 
 * #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
 * struct Project {
 *     name: String,
 *     path: String,
 *     plugins: Vec<String>,
 * }
 * 
 * #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
 * struct RecentProject {
 *     name: String,
 *     path: String,
 * }
 */
