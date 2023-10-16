import { invoke } from '@tauri-apps/api/tauri';

type RequestOptions = unknown;

export async function request(data: RequestOptions) {
	const result = await invoke('request', { data });
	return result;
}
