<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import * as monaco from 'monaco-editor';

	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
	import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
	import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
	import scssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
	import { emmetHTML, emmetCSS, type Dispose } from 'emmet-monaco-es';

	let disposeEmmetHTML: Dispose;
	let disposeEmmetCSS: Dispose;

	let editorContainer: HTMLDivElement;
	let editor: monaco.editor.IStandaloneCodeEditor;

	export let editorValue = ``;

	export let language = 'javascript';
	export let style = '';
	let className = '';
	export { className as class };

	$: if (editor) {
		editor.setValue(editorValue);
	}

	const resizeObserver = new ResizeObserver(() => {
		if (editor) {
			editor.layout({ width: 0, height: 0 });
			editor.layout();
		}
	});

	onMount(async () => {
		disposeEmmetHTML = emmetHTML(monaco, ['html', 'xhtml', 'xml', 'jsx']);
		disposeEmmetCSS = emmetCSS(monaco, ['css', 'scss', 'sass', 'less']);

		monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
			target: monaco.languages.typescript.ScriptTarget.ES2020,
			allowNonTsExtensions: true,
			moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
			module: monaco.languages.typescript.ModuleKind.CommonJS,
			noEmit: true,
			typeRoots: ['node_modules/@types']
		});

		self.MonacoEnvironment = {
			getWorker: function (moduleId, label) {
				if (label === 'typescript' || label === 'javascript') {
					return new tsWorker();
				}
				if (label === 'json') {
					return new jsonWorker();
				}
				if (label === 'html') {
					return new htmlWorker();
				}
				if (label === 'scss' || label === 'css') {
					return new scssWorker();
				}
				return new editorWorker();
			}
		};

		editor = monaco.editor.create(editorContainer, {
			value: editorValue,
			language,
			automaticLayout: true,
			theme: 'vs-dark',
			minimap: {
				enabled: false
			},
		});

		resizeObserver.observe(editorContainer);

		editor.onDidChangeModelContent(handleEditorValueChange);
	});

	onDestroy(() => {
		resizeObserver.unobserve(editorContainer);
		disposeEmmetHTML();
		disposeEmmetCSS();
		editor.dispose();
	});

	function handleEditorValueChange() {
		editorValue = editor.getValue();
	}
</script>

<div bind:this={editorContainer} {style} class={"resize-x overflow-x-auto w-full"} />
