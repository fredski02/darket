import vue from "@vitejs/plugin-vue"
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';
import { defineConfig } from 'vite';
import { PrimeVueResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'

const localNetwork = 'local';
const network = process.env['DFX_NETWORK'] || localNetwork;

let canisterIdPath: string;
if (network === localNetwork) {
  // Local replica canister IDs
  canisterIdPath = join(__dirname, '.dfx/local/canister_ids.json');
} else {
  // Custom canister IDs
  canisterIdPath = join(__dirname, 'canister_ids.json');
}

if (!existsSync(canisterIdPath)) {
  throw new Error(
    'Unable to find canisters. Running `dfx deploy` should fix this problem.',
  );
}
const canisterIds = JSON.parse(readFileSync(canisterIdPath, 'utf8'));

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), Components({
    resolvers : [
      PrimeVueResolver()
    ]
  })],
  define: {
    'process.env.DFX_NETWORK': JSON.stringify(process.env.DFX_NETWORK),
    // Expose canister IDs provided by `dfx deploy`
    ...Object.fromEntries(
      Object.entries(canisterIds).map(([name, ids]) => [
        `process.env.${name.toUpperCase()}_CANISTER_ID`,
        JSON.stringify(ids[network] || ids[localNetwork]),
      ]),
    ),
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: 'globalThis',
      },
    },
  },
  server: {
    // Local IC replica proxy
    fs: {
      allow: ["."],
    },
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
      },
    },
  },
});
