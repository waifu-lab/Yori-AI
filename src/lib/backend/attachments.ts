import type { Attachment } from './types';

function arrayBufferToBase64(buffer: ArrayBuffer): string {
	const bytes = new Uint8Array(buffer);
	const chunkSize = 0x8000;
	let binary = '';
	for (let i = 0; i < bytes.length; i += chunkSize) {
		const chunk = bytes.subarray(i, i + chunkSize);
		binary += String.fromCharCode(...chunk);
	}
	return btoa(binary);
}

export async function fileToAttachment(file: File): Promise<Attachment> {
	const buffer = await file.arrayBuffer();
	return {
		filename: file.name,
		content_type: file.type || 'application/octet-stream',
		data: arrayBufferToBase64(buffer)
	};
}

export function filesToAttachments(files: File[]): Promise<Attachment[]> {
	return Promise.all(files.map(fileToAttachment));
}
