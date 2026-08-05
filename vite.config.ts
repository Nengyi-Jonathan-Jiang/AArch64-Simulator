import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

import * as path from "node:path";

// https://vite.dev/config/
export default defineConfig({
    plugins: [react()],
    appType: 'mpa',
    resolve: {
        alias: {
            '@simulator': path.resolve(__dirname, 'simulator/pkg'),
        }
    },
    build: {
        minify: false,
    },
    base: "https://njonathanj.com/AArch64-Simulator/"
})
