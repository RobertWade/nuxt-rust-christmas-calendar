import { defineEventHandler, readBody } from 'h3'

type SupportedMethod =
  | 'GET'
  | 'HEAD'
  | 'POST'
  | 'PUT'
  | 'DELETE'
  | 'PATCH'
  | 'OPTIONS'
  | 'TRACE'

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const backendUrl = config.public.backendUrl
  const params = event.context.params as { id: string }
  const method = (event.node.req.method || 'GET').toUpperCase() as SupportedMethod

  const headers = Object.fromEntries(
    Object.entries(event.node.req.headers || {}).map(([key, value]) => [key, String(value)]),
  )

  const hasBody = !['GET', 'HEAD'].includes(method)
  const body = hasBody ? await readBody(event) : undefined

  const targetUrl = `${backendUrl}/calendars/${params.id}`

  return await $fetch(targetUrl, {
    method,
    headers,
    body,
  })
})
