import { writable } from 'svelte/store';

export interface Project {
    name: string;
    path: string;
    plugins: string[];
}

function createProjectStore() {
    const { subscribe, set, update } = writable<Project | null>(null);

    return {
        subscribe,
        setProject: (project: Project) => set(project),
        clearProject: () => set(null),
        addPlugin: (pluginName: string) => update(p => {
            if (p) {
                return { ...p, plugins: [...p.plugins, pluginName] };
            }
            return p;
        }),
        removePlugin: (pluginName: string) => update(p => {
            if (p) {
                return { ...p, plugins: p.plugins.filter(name => name !== pluginName) };
            }
            return p;
        })
    };
}

export const currentProject = createProjectStore();
