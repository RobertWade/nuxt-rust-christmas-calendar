export interface User {
  id: string
  name: string
  email?: string
  avatarUrl?: string | null
}

export interface AuthSession {
  token: string
  user: User
  expiresAt: string
}

export interface SignInPayload {
  email: string
  password: string
}

export interface SignUpPayload extends SignInPayload {
  name: string
}
