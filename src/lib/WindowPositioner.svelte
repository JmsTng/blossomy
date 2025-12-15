<script lang="ts">
  import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  async function positionWindow(position: Position) {
    try {
      const window = getCurrentWindow();
      await moveWindow(window, position);
      console.log(`Window moved to ${position}`);
    } catch (error) {
      console.error('Error positioning window:', error);
    }
  }

  // You can also use specific coordinates
  async function moveToCustomPosition(x: number, y: number) {
    try {
      const window = getCurrentWindow();
      await window.setPosition({ x, y });
      console.log(`Window moved to ${x}, ${y}`);
    } catch (error) {
      console.error('Error moving window:', error);
    }
  }
</script>

<div class="window-positioner">
  <h3>Window Positioning</h3>

  <div class="position-buttons">
    <button on:click={() => positionWindow(Position.TopLeft)}>
      Top Left
    </button>
    <button on:click={() => positionWindow(Position.TopCenter)}>
      Top Center
    </button>
    <button on:click={() => positionWindow(Position.TopRight)}>
      Top Right
    </button>
    <button on:click={() => positionWindow(Position.Center)}>
      Center
    </button>
    <button on:click={() => positionWindow(Position.BottomLeft)}>
      Bottom Left
    </button>
    <button on:click={() => positionWindow(Position.BottomCenter)}>
      Bottom Center
    </button>
    <button on:click={() => positionWindow(Position.BottomRight)}>
      Bottom Right
    </button>
  </div>

  <div class="custom-position">
    <h4>Custom Position</h4>
    <button on:click={() => moveToCustomPosition(100, 100)}>
      Move to (100, 100)
    </button>
    <button on:click={() => moveToCustomPosition(500, 300)}>
      Move to (500, 300)
    </button>
  </div>
</div>

<style>
  .window-positioner {
    padding: 20px;
  }

  .position-buttons {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
    margin-bottom: 20px;
  }

  button {
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background: #f9f9f9;
    cursor: pointer;
  }

  button:hover {
    background: #e9e9e9;
  }

  .custom-position {
    border-top: 1px solid #eee;
    padding-top: 20px;
  }
</style>
