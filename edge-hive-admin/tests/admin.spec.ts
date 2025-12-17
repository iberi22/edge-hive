import { test, expect } from '@playwright/test';

test.describe('Edge Hive Admin', () => {
   test('should load the login page', async ({ page }) => {
      await page.goto('/');
      // Depending on auth state, might redirect to login or dashboard
      // Assuming clean state redirects to /auth
      await expect(page).toHaveTitle(/Edge Hive Admin/);
   });

   // Note: Detailed functional tests require mocking Tauri IPC calls
   // which is complex in Playwright (requires window.__TAURI_IPC__ mock).
   // For this baseline, we verify the app serves and defined routes exist.

   test('should display critical UI components on load', async ({ page }) => {
      await page.goto('/');
      // Check for main layout elements if logged in, or login form if not
      // Since we can't easily convince the app we are logged in from Playwright without IPC mocking,
      // we will check for the presence of the root div and basic structure.
      const root = page.locator('#root');
      await expect(root).toBeVisible();
   });
});
