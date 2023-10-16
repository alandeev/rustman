<script>
	import { expandHook } from "~/lib/components/hooks";
	import Menubar from "./_components/Menubar.svelte";
	import Navigator from "./_components/Navigator.svelte";
	import RequestsView from "./_components/RequestsView.svelte";
</script>

<main id="main" class="h-full w-full flex flex-col">
	<Menubar />
	<div id="grid-main" class="h-full w-full flex">
		<div class="left-panel panel disable-select" id="left-panel-side">
			<Navigator />
			<div class="handler-wrapper y-handler-border">
				<div
					class="handler y-handler"
					id="side-menu-left-handler"
					use:expandHook={{ elementSelected: 'side-menu-left', expansionType: 'height' }}
				/>
			</div>
			<div class="left-tabs sub-panel">
				<div class="section-title">My Requests</div>
				<RequestsView />
			</div>
		</div>
		<div class="handler-wrapper x-handler-border">
			<div
				class="handler x-handler"
				id="side-menu-tall-handler"
				use:expandHook={{ elementSelected: 'left-panel-side', expansionType: 'width-l' }}
			/>
		</div>
		<div class="right-panel panel disable-select" id="side-panel-menu-container">
			<div class="main-panel panel" id="upper-right-panel">
				<slot />
			</div>
			<div class="handler-wrapper y-handler-border">
				<div
					class="handler y-handler"
					use:expandHook={{ elementSelected: 'upper-right-panel', expansionType: 'height' }}
				/>
			</div>
			<!-- <div class="lower-panel panel">
				<Terminal />
			</div> -->
		</div>
	</div>
</main>


<style lang="scss">
	.left-panel {
		height: 100%;
		display: flex;
		flex-direction: column;
		flex: 0 0 auto;
	}

	.left-panel {
		/* background-color: #181818; */
		height: 100%;
		display: flex;
		flex-direction: column;
		flex: 0 0 auto;
		overflow: hidden;
		max-width: 25%;
		min-width: 3em;
		width: 10%;
	}

	.right-panel {
		/* width: 85%; */
		height: 100%;
		display: flex;
		flex-direction: column;
		flex: 1 1 auto;
		overflow: hidden;
	}

	/* .lower-panel {
		height: 25%;
		display: flex;
		flex-flow: column;
		flex: 1 1 auto;
		overflow: hidden;
	} */

	:global(.panel) {
		/* flex: auto; */
		margin: 0;
	}

	:global(.sub-panel) {
		box-sizing: border-box;
		padding: 1em;
		padding-bottom: 0;
		/* flex: 0 1 auto; */
		/* border: .5px solid white; */
	}

	.left-tabs {
		flex: 1 1 0;
		overflow: auto;
		overflow-x: hidden;
	}

	#side-menu-left {
		max-height: 70%;
		height: 30%;
		min-height: 30%;
		padding: 0;
	}

	:global(.section-title) {
		/* margin-bottom: .5em; */
		text-transform: uppercase;
		white-space: nowrap;
		padding: 0 0.8em 0.8em 0;
	}

	.main-panel {
		width: 100%;
		height: 75%;
		min-height: 30%;
		max-height: 99%;
		/* background-color: #161616; */
		display: flex;
		flex-flow: row;
		flex: 0 0 auto;
	}

	.left-panel > * {
		overflow: hidden;
	}
</style>
