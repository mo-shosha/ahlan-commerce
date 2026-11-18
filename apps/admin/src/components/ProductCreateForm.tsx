import { useEffect, useId, useRef, useState } from 'react'
import type { FormEvent } from 'react'
import { useCreateProduct } from '../hooks/useCreateProduct'

const emptyForm = {
  title: '',
  handle: '',
  description: '',
  priceCents: '',
  inventoryQuantity: '',
  published: false,
}

type ProductCreateFormProps = {
  open: boolean
  onClose: () => void
}

export function ProductCreateForm({ open, onClose }: ProductCreateFormProps) {
  const createProduct = useCreateProduct()
  const [form, setForm] = useState(emptyForm)
  const titleId = useId()
  const firstFieldRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (!open) return

    setForm(emptyForm)
    createProduct.reset()
    firstFieldRef.current?.focus()

    function onKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape' && !createProduct.isPending) {
        onClose()
      }
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [open])

  if (!open) return null

  function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()

    createProduct.mutate(
      {
        title: form.title.trim(),
        handle: form.handle.trim(),
        description: form.description.trim() || null,
        priceCents: Number(form.priceCents),
        inventoryQuantity: Number(form.inventoryQuantity),
        published: form.published,
      },
      {
        onSuccess: () => {
          setForm(emptyForm)
          onClose()
        },
      },
    )
  }

  return (
    <div
      className="dialog-backdrop"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !createProduct.isPending) {
          onClose()
        }
      }}
    >
      <div
        className="dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
      >
        <div className="dialog-header">
          <h2 id={titleId}>New product</h2>
          <button
            type="button"
            className="icon-button"
            onClick={onClose}
            disabled={createProduct.isPending}
            aria-label="Close"
          >
            ×
          </button>
        </div>

        <form onSubmit={onSubmit} className="product-form">
          <label>
            Title
            <input
              ref={firstFieldRef}
              name="title"
              value={form.title}
              onChange={(e) => setForm({ ...form, title: e.target.value })}
              required
            />
          </label>
          <label>
            Handle
            <input
              name="handle"
              value={form.handle}
              onChange={(e) => setForm({ ...form, handle: e.target.value })}
              required
              pattern="[a-z0-9]+(-[a-z0-9]+)*"
              title="Lowercase letters, digits, and single hyphens"
            />
          </label>
          <label>
            Description
            <textarea
              name="description"
              value={form.description}
              onChange={(e) => setForm({ ...form, description: e.target.value })}
              rows={3}
            />
          </label>
          <div className="form-row">
            <label>
              Price (cents)
              <input
                name="price_cents"
                type="number"
                min={0}
                step={1}
                value={form.priceCents}
                onChange={(e) => setForm({ ...form, priceCents: e.target.value })}
                required
              />
            </label>
            <label>
              Inventory quantity
              <input
                name="inventory_quantity"
                type="number"
                min={0}
                step={1}
                value={form.inventoryQuantity}
                onChange={(e) =>
                  setForm({ ...form, inventoryQuantity: e.target.value })
                }
                required
              />
            </label>
          </div>
          <label className="checkbox">
            <input
              name="published"
              type="checkbox"
              checked={form.published}
              onChange={(e) => setForm({ ...form, published: e.target.checked })}
            />
            Published
          </label>

          {createProduct.isError ? (
            <p role="alert" className="status error">
              {createProduct.error.message}
            </p>
          ) : null}

          <div className="dialog-actions">
            <button
              type="button"
              className="button-secondary"
              onClick={onClose}
              disabled={createProduct.isPending}
            >
              Cancel
            </button>
            <button type="submit" className="button-primary" disabled={createProduct.isPending}>
              {createProduct.isPending ? 'Creating…' : 'Create product'}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
