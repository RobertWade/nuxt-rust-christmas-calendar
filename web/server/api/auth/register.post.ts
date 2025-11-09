export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const payload = await readBody(event)

  return await $fetch(`${config.public.backendUrl}/auth/register`, {
    method: 'POST',
    body: payload,
  })
})
