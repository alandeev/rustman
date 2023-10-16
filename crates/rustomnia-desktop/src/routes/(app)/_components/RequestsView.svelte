<script>
	import Tree from '~/lib/components/treeview/Tree.svelte';

	let children = [
		{
			name: 'GET products'
		},
		{
			name: 'POST products'
		},
		{
			name: 'DELETE products'
		}
	];
</script>

<div class="requests-tree">
	<Tree name="My ERP" {children} expanded />
</div>
