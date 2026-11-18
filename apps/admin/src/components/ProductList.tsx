import { useProducts } from '../hooks/useProducts'
import type { Product } from '../graphql/products'

function formatCents(cents: number): string {
  return (cents / 100).toLocaleString(undefined, {
    style: 'currency',
    currency: 'USD',
  })
}

function formatWhen(value: string | null): string {
  if (!value) return '—'
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value
  return date.toLocaleString(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short',
  })
}

function ProductRow({ product }: { product: Product }) {
  return (
    <tr>
      <td>
        <div className="cell-title">{product.title}</div>
        <div className="cell-meta">{product.handle}</div>
      </td>
      <td className="cell-muted">{product.description ?? '—'}</td>
      <td>{formatCents(product.priceCents)}</td>
      <td>{product.inventoryQuantity}</td>
      <td>
        <span className={product.published ? 'badge on' : 'badge off'}>
          {product.published ? 'Published' : 'Draft'}
        </span>
      </td>
      <td className="cell-muted">{formatWhen(product.publishedAt)}</td>
      <td className="cell-muted">{formatWhen(product.createdAt)}</td>
      <td className="cell-muted">{formatWhen(product.updatedAt)}</td>
    </tr>
  )
}

type ProductListProps = {
  onCreateClick: () => void
}

export function ProductList({ onCreateClick }: ProductListProps) {
  const products = useProducts()

  if (products.isPending) {
    return (
      <div className="panel state-panel" role="status">
        Loading products…
      </div>
    )
  }

  if (products.isError) {
    return (
      <div className="panel state-panel" role="alert">
        <p className="status error">
          Failed to load products: {products.error.message}
        </p>
      </div>
    )
  }

  if (products.data.length === 0) {
    return (
      <div className="panel empty-panel">
        <h2>No products yet</h2>
        <p>Add the first item to your catalog.</p>
        <button type="button" className="button-primary" onClick={onCreateClick}>
          New product
        </button>
      </div>
    )
  }

  return (
    <div className="panel table-wrap">
      <table>
        <thead>
          <tr>
            <th>Product</th>
            <th>Description</th>
            <th>Price</th>
            <th>Inventory</th>
            <th>Status</th>
            <th>Published</th>
            <th>Created</th>
            <th>Updated</th>
          </tr>
        </thead>
        <tbody>
          {products.data.map((product) => (
            <ProductRow key={product.id} product={product} />
          ))}
        </tbody>
      </table>
    </div>
  )
}
