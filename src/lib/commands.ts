import { invoke } from '@tauri-apps/api/core';

export function openLive2dWindow(): Promise<void> {
	return invoke('open_live2d_window');
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
