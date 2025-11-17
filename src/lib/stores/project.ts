/**
 * Stores
 */

import { writable } from 'svelte/store';
import type { ProjectData } from '../commands';

export const projects = writable<ProjectData[]>([]);
export const currentProject = writable<ProjectData | null>(null);
