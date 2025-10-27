import tailwindcss from '@tailwindcss/vite'; //tailwindcss4 专用。
import { paraglideVitePlugin } from '@inlang/paraglide-js';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';
import { FileSystemIconLoader } from 'unplugin-icons/loaders';

export default defineConfig({
	plugins: [
		sveltekit(),
		tailwindcss(),
		Icons({
			compiler: 'svelte',
			autoInstall: true,
			customCollections: {
				// 'my-icons' 是你自定义图标集合的名称
				'fs-icons': FileSystemIconLoader('./assets/icons', (svg) =>
					svg.replace(/^<svg /, '<svg fill="currentColor" ')
				)
			}
		}),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide'
		})
	],
	optimizeDeps: {
		exclude: ['$lib/paraglide/runtime.js', '$lib/paraglide/messages.js']
	},
	build: {
		emptyOutDir: true, target: 'es2021', minify: 'esbuild',
	}
});
