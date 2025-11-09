export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const payload = await readBody(event)

  await $fetch(`${config.public.backendUrl}/auth/password/reset`, {
    method: 'POST',
    body: payload,
  })

  return { success: true }
})
