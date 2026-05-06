import { emit } from '@tauri-apps/api/event';
import { getLive2dTransform, setLive2dTransform } from '$lib/commands';

class Live2dStore {
	scale = $state(1.0);
	offsetX = $state(0.0);
	offsetY = $state(0.0);
	loaded = $state(false);

	async load() {
		if (this.loaded) return;
		const t = await getLive2dTransform();
		this.scale = t.scale;
		this.offsetX = t.offset_x;
		this.offsetY = t.offset_y;
		this.loaded = true;
	}

	preview() {
		if (!this.loaded) return;
		emit('set-live2d-transform', {
			scale: this.scale,
			offset_x: this.offsetX,
			offset_y: this.offsetY
		}).catch(console.warn);
	}

	async save() {
		await setLive2dTransform(this.scale, this.offsetX, this.offsetY);
	}

	reset() {
		this.scale = 1.0;
		this.offsetX = 0.0;
		this.offsetY = 0.0;
		this.preview();
	}
}

export const live2dStore = new Live2dStore();