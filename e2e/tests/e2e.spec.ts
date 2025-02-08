import { test, expect } from '@playwright/test'

test('it renders index page', async ({ page }) => {
  await page.goto('/')
  const header = await page.textContent('h1')
  expect(header).toContain('TUONO')
})

test('it renders /:name page', async ({ page }) => {
  await page.goto('/pokemons/bulbasaur')
  const header = await page.textContent('h1')
  expect(header).toContain('bulbasaur')
})

test('it routes to /:name page on click', async ({ page }) => {
  await page.goto('/')
  const link = page.locator('a[href="/pokemons/bulbasaur"]')
  await link.click()
  await page.waitForURL('/pokemons/bulbasaur')
  const header = await page.textContent('h1')
  expect(header).toContain('bulbasaur')
})
