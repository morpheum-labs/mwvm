import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react({
            // Fast Refresh is enabled by default in Vite + React
        }),
    ],

    // Server configuration for local development
    server: {
        port: 5173,
        open: true,                    // Automatically open browser on dev start
        cors: true,                    // Allow cross-origin requests (useful for testing)
    },

    // Build configuration
    build: {
        target: 'es2020',              // Modern browsers + good WASM support
        outDir: 'dist',
        sourcemap: true,               // Helpful for debugging in production builds
        rollupOptions: {
            output: {
                manualChunks: {
                    // Keep React + ReactDOM in their own chunk for better caching
                    vendor: ['react', 'react-dom'],
                },
            },
        },
    },

    // Resolve configuration (optional but useful for clean imports)
    resolve: {
        alias: {
            // Optional: You can add aliases here if needed in the future
            // '@': '/src',
        },
    },

    // Optimize dependencies (Vite automatically handles WASM)
    optimizeDeps: {
        // No special config needed for mwvm-wasm — Vite handles .wasm files natively
    },
});