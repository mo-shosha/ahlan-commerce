import { graphqlRequest } from './client'

export type Product = {
  id: string
  title: string
  handle: string
  description: string | null
  priceCents: number
  inventoryQuantity: number
  published: boolean
  publishedAt: string | null
  createdAt: string
  updatedAt: string
}

export type ProductCreateInput = {
  title: string
  handle: string
  description?: string | null
  priceCents: number
  inventoryQuantity: number
  published: boolean
}

export const PRODUCTS_QUERY_KEY = ['products'] as const

const PRODUCTS_QUERY = `
  query Products {
    products {
      id
      title
      handle
      description
      priceCents
      inventoryQuantity
      published
      publishedAt
      createdAt
      updatedAt
    }
  }
`

const PRODUCT_CREATE_MUTATION = `
  mutation ProductCreate($input: ProductCreateInput!) {
    productCreate(input: $input) {
      id
      title
      handle
      description
      priceCents
      inventoryQuantity
      published
      publishedAt
      createdAt
      updatedAt
    }
  }
`

type ProductsData = {
  products: Product[]
}

type ProductCreateData = {
  productCreate: Product
}

export async function fetchProducts(): Promise<Product[]> {
  const result = await graphqlRequest<ProductsData>(PRODUCTS_QUERY)
  return result.products
}

export async function createProduct(input: ProductCreateInput): Promise<Product> {
  const result = await graphqlRequest<ProductCreateData>(PRODUCT_CREATE_MUTATION, {
    input,
  })
  return result.productCreate
}
