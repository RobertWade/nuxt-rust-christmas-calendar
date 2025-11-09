export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const backendUrl = config.public.backendUrl // Define this in your nuxt.config.ts

  const response = await $fetch(`${backendUrl}/calendars`, {
    method: event.node.req.method as | "GET" | "HEAD" | "PATCH" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "get" | "head" | "patch" | "post" | "put" | "delete" | "connect" | "options" | "trace" | undefined,
    headers: Object.fromEntries(Object.entries(event.node.req.headers || {}).map(([key, value]) => [key, String(value)])),
    body: event.node.req.method === 'POST' ? await readBody(event) : undefined,
  })

  return response
})