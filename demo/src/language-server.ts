import { TowerLspWasmBridge,create_mumps_lsp } from '../wasm_lsp/pkg/wasm_lsp.js';

let lspServer: TowerLspWasmBridge | null = null;

lspServer = create_mumps_lsp((message: any) => {
    self.postMessage(message);
  });

self.onmessage = (event: MessageEvent) => {
    if (lspServer && event.data) {
    lspServer.handle_incoming_message(event.data);
  }
};
