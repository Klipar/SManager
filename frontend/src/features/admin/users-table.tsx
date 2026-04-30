import { Button } from "@/components/ui/button"
import { Edit2, Plus, Trash2 } from "lucide-react"
import type { AdminUser } from "./types"

interface UsersTableProps {
  users: AdminUser[]
  onEditUser: (user: AdminUser) => void
  onAddUser: () => void
  onDeleteUser: (user: AdminUser) => void
}

export function UsersTable({ users, onEditUser, onAddUser, onDeleteUser }: UsersTableProps) {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-medium text-white">Users</h2>
        <Button
          onClick={onAddUser}
          className="bg-emerald-600 shadow-md transition-all hover:scale-105 hover:bg-emerald-700"
        >
          <Plus className="mr-2 size-4" />
          Add User
        </Button>
      </div>

      <div className="overflow-hidden rounded-xl border border-white/[0.04] bg-white/[0.02]">
        <table className="w-full">
          <thead>
            <tr className="border-b border-white/[0.04] bg-white/[0.025]">
              <th className="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-white/60">
                Role
              </th>
              <th className="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-white/60">
                Name
              </th>
              <th className="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-white/60">
                Email
              </th>
              <th className="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-white/60">
                Last Active
              </th>
              <th className="px-6 py-4 text-left text-xs font-medium uppercase tracking-wider text-white/60">
                Created At
              </th>
              <th className="px-6 py-4 text-right text-xs font-medium uppercase tracking-wider text-white/60">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/[0.04]">
            {users.map((user) => (
              <tr
                key={user.id}
                className="hover:bg-white/[0.03] transition-colors"
              >
                <td className="px-6 py-4">
                  <span className="inline-flex items-center rounded-md bg-white/[0.05] px-3 py-1.5 text-xs font-medium text-white/80">
                    {user.role.toUpperCase()}
                  </span>
                </td>
                <td className="px-6 py-4 text-sm text-white/85">{user.name}</td>
                <td className="px-6 py-4 text-sm text-white/70">{user.email}</td>
                <td className="px-6 py-4 text-sm text-white/60">{user.lastLogin || "Never"}</td>
                <td className="px-6 py-4 text-sm text-white/60">{user.lastUpdate}</td>
                <td className="px-6 py-4">
                  <div className="flex items-center justify-end gap-2">
                    <Button
                      variant="secondary"
                      size="sm"
                      onClick={() => onEditUser(user)}
                      className="h-8 w-8 rounded-xl border-white/10 bg-white/[0.05] p-0 text-white/70 hover:bg-white/[0.08] hover:text-white"
                    >
                      <Edit2 className="size-3.5" />
                    </Button>
                    <Button
                      variant="secondary"
                      size="sm"
                      onClick={() => onDeleteUser(user)}
                      className="h-8 w-8 rounded-xl border-red-500/20 bg-red-950/20 p-0 text-red-400/70 hover:bg-red-950/40 hover:text-red-400"
                    >
                      <Trash2 className="size-3.5" />
                    </Button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
