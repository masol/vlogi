import { paraglideVitePlugin } from '@inlang/paraglide-js';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from "unplugin-icons/vite";
import { FileSystemIconLoader } from "unplugin-icons/loaders";
import UnoCSS from 'unocss/vite'


export default defineConfig({
	plugins: [
		UnoCSS(),
		sveltekit(),
		Icons({
			compiler: "svelte",
			autoInstall: true,
			customCollections: {
				// 'my-icons' 是你自定义图标集合的名称
				"fs-icons": FileSystemIconLoader("./assets/icons", (svg) =>
					svg.replace(/^<svg /, '<svg fill="currentColor" ')
				),
			},
		}),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide',
		})
	],
	build: {
		emptyOutDir: true,
		target: "es2021"
	}
});
