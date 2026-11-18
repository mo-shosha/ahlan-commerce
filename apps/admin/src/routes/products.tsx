import { useState } from 'react'
import { ProductCreateForm } from '../components/ProductCreateForm'
import { ProductList } from '../components/ProductList'

export function ProductsPage() {
  const [createOpen, setCreateOpen] = useState(false)

  return (
    <main className="page">
      <header className="page-header">
        <div>
          <p className="eyebrow">Catalog</p>
          <h1>Products</h1>
          <p className="lede">Manage what you sell. Create once, list updates live.</p>
        </div>
        <button
          type="button"
          className="button-primary"
          onClick={() => setCreateOpen(true)}
        >
          New product
        </button>
      </header>

      <ProductList onCreateClick={() => setCreateOpen(true)} />

      <ProductCreateForm
        open={createOpen}
        onClose={() => setCreateOpen(false)}
      />
    </main>
  )
}
