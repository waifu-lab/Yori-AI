import { invoke } from '@tauri-apps/api/core';

class SettingsStore {
	backendUrl = $state('ws://127.0.0.1:8765');
	live2dModelPath = $state<string | null>(null);
	backendConnected = $state(false);

	private loaded = false;

	async load() {
		if (this.loaded) return;
		this.loaded = true;

		const [url, modelPath, connected] = await Promise.all([
			invoke<string>('backend_get_url'),
			invoke<string | null>('get_model_path'),
			invoke<boolean>('backend_status')
		]);

		this.backendUrl = url;
		this.live2dModelPath = modelPath;
		this.backendConnected = connected;
	}

	async setBackendUrl(url: string) {
		await invoke('backend_set_url', { url });
		this.backendUrl = url;
	}

	async setLive2dModelPath(path: string) {
		await invoke('set_model_path', { path });
		this.live2dModelPath = path;
	}

	async connect() {
		await invoke('backend_connect');
		this.backendConnected = await invoke<boolean>('backend_status');
	}

	async disconnect() {
		await invoke('backend_disconnect');
		this.backendConnected = false;
	}
}

export const settingsStore = new SettingsStore();
