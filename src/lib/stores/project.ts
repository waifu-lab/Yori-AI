import { writable } from 'svelte/store';
import * as commands from '$lib/commands';

export interface Project {
    name: string;
    path: string;
    plugins: string[];
}

function createProjectStore() {
    const { subscribe, set, update } = writable<Project | null>(null);

    return {
        subscribe,
        setProject: async (project: Project) => {
            set(project);
            await commands.saveCurrentProject(project);
        },
        clearProject: async () => {
            set(null);
            await commands.clearCurrentProject();
        },
        loadProject: async () => {
            const project = await commands.loadCurrentProject();
            if (project) {
                set(project);
            }
            return project;
        },
        addPlugin: async (pluginName: string) => {
            let updatedProject: Project | null = null;
            update(p => {
                if (p) {
                    updatedProject = { ...p, plugins: [...p.plugins, pluginName] };
                    return updatedProject;
                }
                return p;
            });
            if (updatedProject) {
                await commands.saveCurrentProject(updatedProject);
            }
        },
        removePlugin: async (pluginName: string) => {
            let updatedProject: Project | null = null;
            update(p => {
                if (p) {
                    updatedProject = { ...p, plugins: p.plugins.filter(name => name !== pluginName) };
                    return updatedProject;
                }
                return p;
            });
            if (updatedProject) {
                await commands.saveCurrentProject(updatedProject);
            }
        }
    };
}

export const currentProject = createProjectStore();

// 最近專案的管理 - 直接導出 commands
export const getRecentProjects = commands.getRecentProjects;
export const addRecentProject = commands.addRecentProject;
export const removeRecentProject = commands.removeRecentProject;
export const updateRecentProjectName = commands.updateRecentProjectName;
