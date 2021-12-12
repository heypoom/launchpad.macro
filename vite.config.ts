import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import macros from 'vite-plugin-babel-macros'
import refresh from '@vitejs/plugin-react-refresh'

// https://vitejs.dev/config/
export default defineConfig({
  esbuild: {
    jsxFactory: 'jsx',
    jsxInject: `
      import { jsx } from '@emotion/react'
    `
  },
  plugins: [react(), macros(), refresh()],
  define: {
    'process.env': {}
  }
})
