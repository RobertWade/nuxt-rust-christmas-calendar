# Route Structure Documentation

## Authentication Middleware

All routes under `/account` are protected by the global authentication middleware (`web/app/middleware/auth.global.ts`). Unauthenticated users attempting to access these routes will be redirected to `/signup`.

## Public Routes

- `/` - Landing page
- `/signup` - User authentication page

## Protected Routes (under `/account`)

All protected routes follow a resource-first naming convention with kebab-case and Nuxt conventions.

### Calendar Management

- `/account/calendars/new` - Create a new calendar
- `/account/calendars/[calendarId]` - View calendar (shows all doors)
- `/account/calendars/[calendarId]/presents` - Edit presents for doors
- `/account/calendars/[calendarId]/presents/[door]/edit` - Edit a specific door's present
- `/account/calendars/[calendarId]/doors/[day]/open` - Open a specific door

## Route Parameters

- `[calendarId]` - The unique identifier of a calendar (e.g., `cal-1234567890`)
- `[door]` - The door number for editing presents (1-24)
- `[day]` - The day number for opening doors (1-24)

## Implementation Notes

- Calendar IDs are generated client-side and stored in Pinia with persistence
- The middleware checks for `userStore.user` existence
- All navigation uses programmatic routing via `router.push()` with dynamic calendar IDs
- Route params are used instead of query params for RESTful URLs
