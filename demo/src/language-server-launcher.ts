import { ConnectionConfig } from 'monaco-languageclient/lcwrapper';
import { BrowserMessageReader, BrowserMessageWriter } from 'vscode-languageclient/browser';

export function start_language_server(): ConnectionConfig {
	const worker = new Worker(new URL('./language-server.ts', import.meta.url), {
	  type: 'module',
	})

	const reader = new BrowserMessageReader(worker);
	const writer = new BrowserMessageWriter(worker);

	return {
		options: {
			$type: 'WorkerDirect',
			worker
		},
		messageTransports: { reader, writer }
	}
}
