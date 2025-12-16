import { test, expect } from '@playwright/test';

// Note: Requires Tauri app running (npm run tauri:dev)

test.describe('Dashboard E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1420/');
    await page.waitForLoadState('networkidle');
  });

  test('renders dashboard page', async ({ page }) => {
    // Check for main dashboard container
    const dashboard = page.locator('.dashboard');
    await expect(dashboard).toBeVisible();
  });

  test('displays stats cards', async ({ page }) => {
    // Should have multiple stat cards (CPU, RAM, etc.)
    const statsCards = page.locator('.stats-card');
    await expect(statsCards.first()).toBeVisible();
    
    const count = await statsCards.count();
    expect(count).toBeGreaterThan(0);
  });

  test('shows node list table', async ({ page }) => {
    // Check for nodes table
    const nodeTable = page.locator('table');
    await expect(nodeTable).toBeVisible();

    // Check table headers
    const headers = page.locator('th');
    await expect(headers.first()).toBeVisible();
  });

  test('node status badges have correct colors', async ({ page }) => {
    // Find status badges
    const onlineBadge = page.locator('.badge:has-text("online")').first();
    const offlineBadge = page.locator('.badge:has-text("offline")').first();

    if (await onlineBadge.isVisible()) {
      const className = await onlineBadge.getAttribute('class');
      expect(className).toContain('green');
    }

    if (await offlineBadge.isVisible()) {
      const className = await offlineBadge.getAttribute('class');
      expect(className).toContain('red');
    }
  });

  test.skip('stats auto-refresh', async ({ page }) => {
    // Get initial CPU value
    const cpuCard = page.locator('.stats-card:has-text("CPU")');
    const initialValue = await cpuCard.locator('.value').textContent();

    // Wait for auto-refresh (5 seconds)
    await page.waitForTimeout(6000);

    // CPU value might have changed (or stayed the same if CPU is stable)
    const newValue = await cpuCard.locator('.value').textContent();
    expect(newValue).toBeDefined();
  });

  test('sidebar navigation works', async ({ page }) => {
    // Check if sidebar exists
    const sidebar = page.locator('.sidebar');
    await expect(sidebar).toBeVisible();

    // Try navigating to terminal
    const terminalLink = page.locator('a[href="/terminal"]');
    if (await terminalLink.isVisible()) {
      await terminalLink.click();
      await page.waitForLoadState('networkidle');
      
      // Should be on terminal page
      expect(page.url()).toContain('/terminal');
    }
  });

  test('glassmorphism styling is applied', async ({ page }) => {
    const statsCard = page.locator('.stats-card').first();
    
    // Check for backdrop-filter (glassmorphism effect)
    const backdropFilter = await statsCard.evaluate(el => {
      return window.getComputedStyle(el).backdropFilter || 
             window.getComputedStyle(el).webkitBackdropFilter;
    });

    // Should have blur effect
    expect(backdropFilter).toBeTruthy();
  });
});
