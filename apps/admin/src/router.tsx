import {
  Outlet,
  Link,
  createRootRoute,
  createRoute,
  createRouter,
  redirect,
} from '@tanstack/react-router'
import { ProductsPage } from './routes/products'

function RootLayout() {
  return (
    <div className="app-shell">
      <nav className="top-nav">
        <Link to="/products" className="brand">
          Ahlan Admin
        </Link>
        <Link to="/products">Products</Link>
      </nav>
      <Outlet />
    </div>
  )
}

const rootRoute = createRootRoute({
  component: RootLayout,
})

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  beforeLoad: () => {
    throw redirect({ to: '/products' })
  },
})

const productsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/products',
  component: ProductsPage,
})

const routeTree = rootRoute.addChildren([indexRoute, productsRoute])

export const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
