import { vadStart, vadStop, vadStatus } from '$lib/commands';

class VadStore {
	active = $state(false);
	busy = $state(false);
	error = $state<string | null>(null);

	async init() {
		this.active = await vadStatus().catch(() => false);
	}

	async toggle() {
		this.busy = true;
		this.error = null;
		try {
			if (this.active) {
				await vadStop();
				this.active = false;
			} else {
				await vadStart();
				this.active = true;
			}
		} catch (e) {
			this.error = String(e);
		} finally {
			this.busy = false;
		}
	}

	async start() {
		if (this.active || this.busy) return;
		this.busy = true;
		this.error = null;
		try {
			await vadStart();
			this.active = true;
		} catch (e) {
			this.error = String(e);
		} finally {
			this.busy = false;
		}
	}

	async stop() {
		if (!this.active || this.busy) return;
		this.busy = true;
		this.error = null;
		try {
			await vadStop();
			this.active = false;
		} catch (e) {
			this.error = String(e);
		} finally {
			this.busy = false;
		}
	}
}

export const vadStore = new VadStore();
