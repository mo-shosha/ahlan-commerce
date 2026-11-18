import { expect, test } from '@playwright/test'

test('create product updates the list without a full page refresh', async ({
  page,
}) => {
  const suffix = Date.now().toString(36)
  const title = `E2E Mug ${suffix}`
  const handle = `e2e-mug-${suffix}`

  await page.goto('/products')
  await expect(page.getByRole('heading', { name: 'Products' })).toBeVisible()

  await expect(
    page
      .getByText(/No products yet|Loading products/i)
      .or(page.locator('table')),
  ).toBeVisible({ timeout: 15_000 })

  await page.getByRole('button', { name: 'New product' }).first().click()
  await expect(page.getByRole('dialog', { name: 'New product' })).toBeVisible()

  await page.getByLabel('Title').fill(title)
  await page.getByLabel('Handle').fill(handle)
  await page.getByLabel('Description').fill('Created by Playwright e2e.')
  await page.getByLabel('Price (cents)').fill('1999')
  await page.getByLabel('Inventory quantity').fill('4')
  await page.getByLabel('Published').check()

  await page.getByRole('button', { name: 'Create product' }).click()

  await expect(page.getByRole('dialog')).toHaveCount(0)
  await expect(page.locator('table')).toContainText(title)
  await expect(page.locator('table')).toContainText(handle)
  await expect(page).toHaveURL(/\/products$/)
})
