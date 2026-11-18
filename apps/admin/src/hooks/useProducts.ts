import { useQuery } from '@tanstack/react-query'
import { fetchProducts, PRODUCTS_QUERY_KEY } from '../graphql/products'

export function useProducts() {
  return useQuery({
    queryKey: PRODUCTS_QUERY_KEY,
    queryFn: fetchProducts,
  })
}
