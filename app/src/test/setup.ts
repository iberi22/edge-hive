import '@testing-library/jest-dom';

// Mock Tauri API
global.window = global.window || {};
global.window.__TAURI__ = {
   core: {
      invoke: vi.fn(),
   },
   event: {
      listen: vi.fn(),
      emit: vi.fn(),
   },
};

// Mock xterm
vi.mock('xterm', () => ({
   Terminal: vi.fn(() => ({
      open: vi.fn(),
      write: vi.fn(),
      writeln: vi.fn(),
      onData: vi.fn(),
      dispose: vi.fn(),
      loadAddon: vi.fn(),
   })),
}));

vi.mock('@xterm/addon-fit', () => ({
   FitAddon: vi.fn(() => ({
      fit: vi.fn(),
      proposeDimensions: vi.fn(() => ({ rows: 24, cols: 80 })),
   })),
}));

vi.mock('@xterm/addon-web-links', () => ({
   WebLinksAddon: vi.fn(() => ({})),
}));
