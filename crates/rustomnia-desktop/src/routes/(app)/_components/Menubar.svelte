<script lang="ts">
	import * as Menubar from '$lib/components/ui/menubar';
</script>

<Menubar.Root>
	<Menubar.Menu>
		<Menubar.Trigger>File</Menubar.Trigger>
		<Menubar.Content>
			<Menubar.Item>
				New Project
				<Menubar.Shortcut>⌘T</Menubar.Shortcut>
			</Menubar.Item>
			<Menubar.Item>Open Project</Menubar.Item>
			<Menubar.Item>Close Project</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Close Project</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Print</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Recent Projects</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Exit</Menubar.Item>
		</Menubar.Content>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Edit</Menubar.Trigger>
		<Menubar.Content>
			<Menubar.Item>Cancel</Menubar.Item>
			<Menubar.Item>Undo Edit</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Cut</Menubar.Item>
			<Menubar.Item>Copy</Menubar.Item>
			<Menubar.Item>Paste</Menubar.Item>
			<Menubar.Separator />
			<Menubar.Item>Properties</Menubar.Item>
		</Menubar.Content>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>View</Menubar.Trigger>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Project</Menubar.Trigger>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Options</Menubar.Trigger>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Debug</Menubar.Trigger>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Tools</Menubar.Trigger>
	</Menubar.Menu>
	<Menubar.Menu>
		<Menubar.Trigger>Help</Menubar.Trigger>
	</Menubar.Menu>
</Menubar.Root>