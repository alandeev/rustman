<script context="module" lang="ts">
  interface ExpandHookOptions {
    elementSelected: string;
    expansionType: string;
  }

  export function expandHook(
    currentTarget: HTMLElement,
    { elementSelected, expansionType }: ExpandHookOptions
  ) {
    let currentlyExpanding: HTMLElement | null = null;

    let initial: number = 0;
    let start: number = 0;

    let currentHandler: HTMLElement | null;

    function startExpand(event: MouseEvent) {
      currentlyExpanding = document.getElementById(elementSelected);
      if (currentlyExpanding) {
        currentHandler = event.target as HTMLElement;
        currentHandler.style.opacity = "1";

        if (expansionType === "height") {
          start = event.pageY;
          initial = currentlyExpanding.offsetHeight;
          console.log({ start, initial });
        }
        if (expansionType === "width-r" || expansionType === "width-l") {
          start = event.pageX;
          initial = currentlyExpanding.offsetWidth;
        }

        window.addEventListener("mouseup", handleMouseUp);
        window.addEventListener("mousemove", handleMouseMove);
      }
    }

    function stopExpand(event: MouseEvent) {
      currentlyExpanding = null;
      start = 0;
      initial = 0;

      if (currentHandler) {
        currentHandler.style.opacity = "0";
      }
      currentHandler = null;

      window.removeEventListener("mouseup", handleMouseUp);
      window.removeEventListener("mousemove", handleMouseMove);
    }

    function expand(event: MouseEvent) {
      if (!currentlyExpanding) return;

      if (expansionType == "height") {
        const dY = start - event.pageY;
        currentlyExpanding.style.height = `${initial - dY}px`;
        // window.api.updateTerminalSize();
        return;
      }
      if (expansionType == "width-r" || expansionType == "width-l") {
        // console.log(event.pageX +" " + start + " " + initial);
        const dX = start - event.pageX;
        if (expansionType == "width-r")
          currentlyExpanding.style.width = `${initial + dX}px`;
        else if (expansionType == "width-l")
          currentlyExpanding.style.width = `${initial - dX}px`;

        // window.api.updateTerminalSize();
        return;
      }
    }

    function handleMouseDown(event: MouseEvent) {
      startExpand(event);
    }
    currentTarget.addEventListener("mousedown", handleMouseDown);

    function handleMouseUp(event: MouseEvent) {
      stopExpand(event);
    }

    function handleMouseMove(event: MouseEvent) {
      expand(event);
    }

    return {
      destroy() {
        currentTarget.removeEventListener("mousedown", handleMouseDown);
        window.removeEventListener("mouseup", handleMouseUp);
        window.removeEventListener("mousemove", handleMouseMove);
      },
    };
  }
</script>

<style>
  :global(.handler) {
    padding: 0;
    position: absolute;
    display: block;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: rgb(61, 108, 146);
    opacity: 0;
    transition: opacity 0.2s;
    transition-delay: 200ms;
  }

  :global(.handler:hover) {
    opacity: 1 !important;
  }

  :global(.handler-wrapper) {
    padding: 0;
    position: relative;
    z-index: 20;
    flex: 0 0 auto;
  }

  :global(.x-handler-border) {
    width: 1px;
    background-color: rgb(66, 66, 66);
  }

  :global(.x-handler) {
    width: 0.3em;
    cursor: col-resize;
  }

  :global(.y-handler) {
    height: 0.3em;
    cursor: row-resize;
  }

  :global(.y-handler-border) {
    height: 1px;
    background-color: rgb(66, 66, 66);
  }
</style>
