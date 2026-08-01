import importMetaUrlPlugin from '@codingame/esbuild-import-meta-url-plugin';

export default {
  resolve: {
    dedupe: ['vscode']
  },
  plugins: [
    importMetaUrlPlugin
    // ... other plugins
  ],
  worker: {
    format: 'es'
  },
  optimizeDeps: {
    esbuildOptions: {
      plugins: [importMetaUrlPlugin]
    }
  }
};
