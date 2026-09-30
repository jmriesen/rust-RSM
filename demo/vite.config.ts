import wasm from 'vite-plugin-wasm';

export default {
  plugins: [
    wasm(),
  ],
  worker: {
    format: 'es',
    plugins: () => [wasm() ]
  },
  base:"./"
};
