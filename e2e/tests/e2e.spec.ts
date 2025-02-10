import { test, expect } from '@playwright/test'

test('it renders index page', async ({ page }) => {
  await page.goto('/')
  const header = await page.textContent('h1')
  expect(header).toContain('TUONO')
})
