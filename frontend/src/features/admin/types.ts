export type UserRole = "admin" | "user"

export interface AdminUser {
  id: number
  name: string
  email: string
  role: UserRole
  lastLogin: string | null
  lastUpdate: string
  createdAt?: string
}

export interface EditUserForm {
  name: string
  email: string
  password: string
  role: UserRole
}
