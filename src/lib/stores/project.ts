/**
 * Project Store - 用於存儲當前選中的專案
 * 
 * 這個 store 主要用於在路由之間共享當前選中的專案狀態
 * 實際的數據操作都通過 commands.ts 與後端交互
 */

import { writable } from 'svelte/store';
import type { ProjectData } from '../commands';

/**
 * 當前專案 Store
 */
function createProjectStore() {
    const { subscribe, set } = writable<ProjectData | null>(null);

    return {
        subscribe,
        /**
         * 設置當前選中的專案
         */
        setCurrent: (project: ProjectData | null) => {
            set(project);
        },
        /**
         * 清除當前專案
         */
        clearCurrent: () => {
            set(null);
        }
    };
}

export const projectStore = createProjectStore();
