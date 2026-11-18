import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  createProduct,
  PRODUCTS_QUERY_KEY,
  type ProductCreateInput,
} from '../graphql/products'

export function useCreateProduct() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (input: ProductCreateInput) => createProduct(input),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: PRODUCTS_QUERY_KEY })
    },
  })
}
