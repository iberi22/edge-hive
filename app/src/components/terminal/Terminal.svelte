<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import 'xterm/css/xterm.css';

  let terminalContainer: HTMLElement;
  let terminal: any = null;
  let fitAddon: any = null;
  let unlisten: (() => void) | null = null;

  onMount(async () => {
    // Dynamic import to avoid SSR issues
    const { Terminal } = await import('xterm');
    const { FitAddon } = await import('@xterm/addon-fit');
    const { WebLinksAddon } = await import('@xterm/addon-web-links');

    // Initialize terminal
    terminal = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: '"Fira Code", "JetBrains Mono", Consolas, monospace',
      theme: {
        background: '#020617', // slate-950
        foreground: '#cbd5e1', // slate-300
        cursor: '#f97316', // orange-500
        cursorAccent: '#020617',
        selectionBackground: '#334155', // slate-700
        black: '#020617',
        red: '#ef4444',
        green: '#10b981',
        yellow: '#f97316', // orange-500 (primary)
        blue: '#0ea5e9', // sky-500 (secondary)
        magenta: '#8b5cf6',
        cyan: '#06b6d4',
        white: '#f8fafc',
        brightBlack: '#475569',
        brightRed: '#f87171',
        brightGreen: '#34d399',
        brightYellow: '#fbbf24',
        brightBlue: '#38bdf8',
        brightMagenta: '#a78bfa',
        brightCyan: '#22d3ee',
        brightWhite: '#ffffff',
      },
    });

    // Add addons
    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.loadAddon(new WebLinksAddon());

    // Open terminal in container
    terminal.open(terminalContainer);
    fitAddon.fit();

    // Welcome message
    terminal.writeln('🚀 \x1b[1;36mEdge Hive Terminal\x1b[0m');
    terminal.writeln('Connecting to shell...\n');

    // Connect to PTY backend
    await connectPTY();

    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
      fitAddon?.fit();
      const dims = fitAddon?.proposeDimensions();
      if (dims) {
        invoke('terminal_resize', { rows: dims.rows, cols: dims.cols });
      }
    });
    resizeObserver.observe(terminalContainer);

    return () => {
      resizeObserver.disconnect();
    };
  });

  async function connectPTY() {
    try {
      // Spawn PTY process
      await invoke('terminal_spawn');
      terminal?.writeln('\x1b[1;32m✓\x1b[0m Connected to shell\n');

      // Listen for output from backend
      unlisten = await listen<{ data: string }>('terminal-output', (event) => {
        terminal?.write(event.payload.data);
      });

      // Send input to backend
      terminal?.onData(async (data) => {
        try {
          await invoke('terminal_write', { data });
        } catch (error) {
          console.error('Failed to write to terminal:', error);
        }
      });
    } catch (error) {
      terminal?.writeln(`\x1b[1;31m✗\x1b[0m Failed to connect: ${error}\n`);
    }
  }

  onDestroy(() => {
    unlisten?.();
    terminal?.dispose();
  });
</script>

<div class="terminal-wrapper">
  <div class="terminal-header">
    <div class="window-controls">
      <span class="control close"></span>
      <span class="control minimize"></span>
      <span class="control maximize"></span>
    </div>
    <span class="terminal-title">Terminal</span>
  </div>
  <div bind:this={terminalContainer} class="terminal-container"></div>
</div>

<style>
  .terminal-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: rgba(15, 23, 42, 0.6);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(148, 163, 184, 0.1);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
  }

  .terminal-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background: rgba(30, 41, 59, 0.8);
    border-bottom: 1px solid rgba(148, 163, 184, 0.1);
  }

  .window-controls {
    display: flex;
    gap: 8px;
  }

  .control {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    display: inline-block;
  }

  .control.close {
    background: #ef4444;
  }

  .control.minimize {
    background: #f59e0b;
  }

  .control.maximize {
    background: #10b981;
  }

  .terminal-title {
    margin-left: auto;
    color: #94a3b8;
    font-size: 14px;
    font-weight: 500;
  }

  .terminal-container {
    flex: 1;
    padding: 16px;
    overflow: hidden;
  }

  /* Override xterm defaults for glassmorphism */
  :global(.xterm) {
    padding: 0 !important;
  }

  :global(.xterm-viewport) {
    background-color: transparent !important;
  }

  :global(.xterm-screen) {
    background-color: transparent !important;
  }
</style>
