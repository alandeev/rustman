<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import Button from '~/lib/components/ui/button/button.svelte';
	import Label from '~/lib/components/ui/label/label.svelte';
	import * as Select from '$lib/components/ui/select';
	import { request } from '~/lib/request';
	import * as Tabs from '$lib/components/ui/tabs';
	import CodeEditor from '~/lib/components/code/CodeEditor.svelte';

	const methods = [
		{ value: 'get', label: 'GET' },
		{ value: 'post', label: 'POST' },
		{ value: 'put', label: 'PUT' },
		{ value: 'delete', label: 'DELETE' }
	];

	let requestUrl = 'https://api.github.com/users/hivexdev';
	let requestMethod = 'GET';
	let requestBody = '{}';

	let responsePreview = `{ "hello": "world" }`;

	function sendRequest() {
		request({ method: requestMethod, url: requestUrl }).then((res) => {
			responsePreview = JSON.stringify(res, null, 2);
		});
	}
</script>

<div class="flex flex-col space-y-5 h-full">
	<Select.Root>
		<Select.Trigger class="w-[180px]">
			<Select.Value placeholder="Select a method" />
		</Select.Trigger>
		<Select.Content>
			<Select.Group>
				<Select.Label>Request Method</Select.Label>
				{#each methods as method}
					<Select.Item value={method.value} label={method.label}>{method.label}</Select.Item>
				{/each}
			</Select.Group>
		</Select.Content>
		<Select.Input name="method" bind:value={requestMethod} />
	</Select.Root>

	<div class="flex items-center space-x-2">
		<Label for="url" class="mr-2">URL:</Label>
		<Input type="url" placeholder="Enter URL..." class="max-w-xs" bind:value={requestUrl} />
		<Button on:click={() => sendRequest()}>Send</Button>
	</div>

	<div class="flex flex-1 gap-3 flex-col md:flex-row">
		<Tabs.Root value="body" class="flex flex-col flex-auto">
			<Tabs.List>
				<Tabs.Trigger value="body">Body</Tabs.Trigger>
				<Tabs.Trigger value="authentication">Authentication</Tabs.Trigger>
				<Tabs.Trigger value="query">Query</Tabs.Trigger>
				<Tabs.Trigger value="headers">Headers</Tabs.Trigger>
				<Tabs.Trigger value="docs">Docs</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="body" class="flex m-0 p-0 flex-1">
				<CodeEditor bind:editorValue={requestBody} language="json" class="flex-auto" />
			</Tabs.Content>
			<Tabs.Content value="headers" />
			<Tabs.Content value="cookies" />
			<Tabs.Content value="timeline" />
		</Tabs.Root>

		<Tabs.Root value="preview" class="flex flex-col flex-auto">
			<Tabs.List>
				<Tabs.Trigger value="preview">Preview</Tabs.Trigger>
				<Tabs.Trigger value="headers">Headers</Tabs.Trigger>
				<Tabs.Trigger value="cookies">Cookies</Tabs.Trigger>
				<Tabs.Trigger value="timeline">Timeline</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="preview" class="flex m-0 p-0 flex-auto">
				<CodeEditor bind:editorValue={responsePreview} language="json" class="flex-auto" />
			</Tabs.Content>
			<Tabs.Content value="headers" />
			<Tabs.Content value="cookies" />
			<Tabs.Content value="timeline" />
		</Tabs.Root>
	</div>
</div>
