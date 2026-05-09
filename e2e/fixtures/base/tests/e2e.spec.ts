import { test, expect } from '@playwright/test'

test('it renders index page', async ({ page }) => {
  await page.goto('/')
  const header = await page.textContent('h1')
  const subtitle = await page.textContent('h2')
  expect(header).toContain('TUONO')
  expect(subtitle).toContain('Subtitle received from the server')
})

test('it renders second route', async ({ page }) => {
  await page.goto('/second-route')
  const header = await page.textContent('h1')
  expect(header).toContain('Second route')
})

test('it routes to second route on link click', async ({ page }) => {
  await page.goto('/')
  await page.click('text=Routing link')
  await page.waitForURL('/second-route')
  const header = await page.textContent('h1')
  expect(header).toContain('Second route')
})

test('it reads server-side env variable from .env file', async ({ page }) => {
  await page.goto('/env-vars')
  const serverVar = await page.textContent('[data-testid="server-var"]')
  expect(serverVar).toBe('server_value_from_env')
})

test('it reads TUONO_PUBLIC_ env variable on the server', async ({ page }) => {
  await page.goto('/env-vars')
  const publicVarServer = await page.textContent(
    '[data-testid="public-var-server"]',
  )
  expect(publicVarServer).toBe('public_value_from_env')
})

test('it reads TUONO_PUBLIC_ env variable on the client', async ({ page }) => {
  await page.goto('/env-vars')
  const publicVarClient = await page.textContent(
    '[data-testid="public-var-client"]',
  )
  expect(publicVarClient).toBe('public_value_from_env')
})
