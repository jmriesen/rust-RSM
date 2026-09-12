import importMetaUrlPlugin from '@codingame/esbuild-import-meta-url-plugin';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default {
  plugins: [
    importMetaUrlPlugin,
    wasm(),
    topLevelAwait()
  ],
  worker: {
    format: 'es',
    plugins: () => [wasm(), topLevelAwait()]
  },
  optimizeDeps: {
    exclude: ['@vscode/diff'],
    esbuildOptions: {
      plugins: [importMetaUrlPlugin]
    }
  }
};
