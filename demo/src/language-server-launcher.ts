import { ConnectionConfig } from 'monaco-languageclient/lcwrapper';
import LspWorker from './language-server.ts?worker';
import { BrowserMessageReader, BrowserMessageWriter } from 'vscode-languageclient/browser';

export function start_language_server(): ConnectionConfig{
	const worker = new LspWorker()

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
