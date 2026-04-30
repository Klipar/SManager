import type { AdminUser } from "./types"

export const mockUsers: AdminUser[] = [
  {
    id: 1,
    name: "Klipar",
    email: "klipar@mail.trinity",
    role: "admin",
    lastLogin: "a few seconds ago",
    lastUpdate: "April 7, 2026",
    createdAt: "April 7, 2026",
  },
  {
    id: 2,
    name: "John Developer",
    email: "john.dev@mail.trinity",
    role: "user",
    lastLogin: "2 hours ago",
    lastUpdate: "April 28, 2026",
    createdAt: "April 15, 2026",
  },
  {
    id: 3,
    name: "Jane Manager",
    email: "jane.manager@mail.trinity",
    role: "admin",
    lastLogin: "yesterday",
    lastUpdate: "April 27, 2026",
    createdAt: "March 20, 2026",
  },
  {
    id: 4,
    name: "Bob Tester",
    email: "bob.tester@mail.trinity",
    role: "user",
    lastLogin: "3 days ago",
    lastUpdate: "April 26, 2026",
    createdAt: "February 10, 2026",
  },
]
