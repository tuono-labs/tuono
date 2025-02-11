import { test, expect } from '@playwright/test'

test('it renders index page', async ({ page }) => {
  await page.goto('/')
  const header = await page.textContent('h1')
  expect(header).toContain('TUONO')
})

test('it renders second route', async ({ page }) => {
  await page.goto('/second-route')
  const header = await page.textContent('h1')
  expect(header).toContain('Second route')
})

test('it routes to second route on link click', async ({ page }) => {
  await page.goto('/');
  await page.click('text=Routing link');
  await page.waitForURL('/second-route');
  const header = await page.textContent('h1')
  expect(header).toContain('Second route')
})
