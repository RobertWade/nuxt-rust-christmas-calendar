export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const payload = await readBody(event)

  return await $fetch(`${config.public.backendUrl}/auth/login`, {
    method: 'POST',
    body: payload,
  })
})
