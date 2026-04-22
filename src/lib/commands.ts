import { invoke } from '@tauri-apps/api/core';

export function openLive2dWindow(): Promise<void> {
	return invoke('open_live2d_window');
}

export function openSettingsWindow(): Promise<void> {
	return invoke('open_settings_window');
}

export function closeLive2dWindow(): Promise<void> {
	return invoke('close_live2d_window');
}

export function setModelPath(path: string): Promise<void> {
	return invoke('set_model_path', { path });
}

export function getCurrentModelUrl(): Promise<string | null> {
	return invoke('get_current_model_url');
}

type CommandResult<T> = {
	data: T;
	err: string;
};

export async function readFile(filePath: string): Promise<Uint8Array> {
	const result = await invoke<CommandResult<number[]>>('read_file', { filePath });
	if (result.err) {
		throw new Error(result.err);
	}

	return Uint8Array.from(result.data);
}

export type NodeStatus = 'running' | 'stopped';

export function agentInstall(): Promise<void> {
	return invoke('waifu_node_install');
}

export function agentStart(): Promise<void> {
	return invoke('waifu_node_start');
}

export function agentStop(): Promise<void> {
	return invoke('waifu_node_stop');
}

export function agentStatus(): Promise<NodeStatus> {
	return invoke('waifu_node_status');
}

export function agentIsInstalled(): Promise<boolean> {
	return invoke('waifu_node_is_installed');
}

/**
 * Tell the Rust backend whether the currently loaded Live2D model requires
 * server-side lipsync viseme computation (`true` = no native LipSync group).
 */
export function setLipsyncMode(needBackendLipsync: boolean): Promise<void> {
	return invoke('live2d_set_lipsync_mode', { needBackendLipsync });
}
