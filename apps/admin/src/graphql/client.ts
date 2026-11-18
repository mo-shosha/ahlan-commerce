export type GraphQlError = {
  message: string
  extensions?: {
    code?: string
    request_id?: string
  }
}

export type GraphQlResponse<T> = {
  data?: T
  errors?: GraphQlError[]
}

export class GraphQlRequestError extends Error {
  readonly errors: GraphQlError[]

  constructor(errors: GraphQlError[]) {
    const first = errors[0]?.message ?? 'GraphQL request failed'
    super(first)
    this.name = 'GraphQlRequestError'
    this.errors = errors
  }
}

const GRAPHQL_URL = import.meta.env.VITE_GRAPHQL_URL ?? '/graphql'

export async function graphqlRequest<T>(
  query: string,
  variables?: Record<string, unknown>,
): Promise<T> {
  const response = await fetch(GRAPHQL_URL, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify({ query, variables }),
  })

  if (!response.ok) {
    throw new Error(`GraphQL HTTP ${response.status}`)
  }

  const body = (await response.json()) as GraphQlResponse<T>

  if (body.errors?.length) {
    throw new GraphQlRequestError(body.errors)
  }

  if (body.data === undefined) {
    throw new Error('GraphQL response missing data')
  }

  return body.data
}
