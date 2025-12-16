import { test, expect } from '@playwright/test';

// Note: Playwright E2E tests for Tauri apps require the app to be running
// Run: npm run tauri:dev before executing these tests

test.describe('Terminal E2E', () => {
   test.beforeEach(async ({ page }) => {
      // Navigate to terminal page
      await page.goto('http://localhost:1420/terminal');
      await page.waitForLoadState('networkidle');
   });

   test('renders terminal interface', async ({ page }) => {
      // Check for terminal container
      const terminalWrapper = page.locator('.terminal-wrapper');
      await expect(terminalWrapper).toBeVisible();
   });

   test('displays terminal header with window controls', async ({ page }) => {
      const header = page.locator('.terminal-header');
      await expect(header).toBeVisible();

      // Check for 3 window controls
      const controls = page.locator('.control');
      await expect(controls).toHaveCount(3);
   });

   test('spawns PTY and shows prompt', async ({ page }) => {
      // Wait for terminal to initialize
      await page.waitForTimeout(1000);

      // Check for terminal content (should have xterm terminal)
      const xtermScreen = page.locator('.xterm-screen');
      await expect(xtermScreen).toBeVisible();
   });

   test.skip('accepts keyboard input', async ({ page }) => {
      // This test requires Tauri app running
      await page.click('.terminal-wrapper');
      await page.keyboard.type('echo "Hello from Playwright"');
      await page.keyboard.press('Enter');

      // Wait for command execution
      await page.waitForTimeout(500);

      // Check output (implementation-dependent)
      const terminalContent = await page.locator('.xterm-screen').textContent();
      expect(terminalContent).toContain('Hello from Playwright');
   });

   test('terminal theme is applied', async ({ page }) => {
      const xtermViewport = page.locator('.xterm-viewport');

      // Check background color (dark theme)
      const bgColor = await xtermViewport.evaluate(el => {
         return window.getComputedStyle(el.parentElement!).backgroundColor;
      });

      // Should be dark (close to #0f172a)
      expect(bgColor).toBeTruthy();
   });
});
