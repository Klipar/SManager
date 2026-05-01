import React from "react"
import { sendCoreRequest } from "@/lib/ws"
import { UsersTable } from "./users-table"
import { EditUserModal } from "./edit-user-modal"
import { DeleteUserModal } from "./delete-user-modal"
import type { AdminUser, EditUserForm } from "./types"

export function AdminPanel() {
  const [users, setUsers] = React.useState<AdminUser[]>([])
  const [isLoading, setIsLoading] = React.useState(true)
  const [error, setError] = React.useState<string | null>(null)
  const [editingUser, setEditingUser] = React.useState<AdminUser | null>(null)
  const [isModalOpen, setIsModalOpen] = React.useState(false)
  const [deletingUser, setDeletingUser] = React.useState<AdminUser | null>(null)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = React.useState(false)
  const [isSaving, setIsSaving] = React.useState(false)

  React.useEffect(() => {
    loadUsers()
  }, [])

  const loadUsers = () => {
    setIsLoading(true)
    setError(null)
    sendCoreRequest("get-all-users", null)
      .then((res) => {
        if (res?.status === "ok" && res.data?.users) {
          const mappedUsers: AdminUser[] = res.data.users.map((u: any) => ({
            id: u.id,
            name: u.name,
            email: u.email,
            role: u.is_admin ? "admin" : "user",
            lastLogin: u.last_login ? u.last_login.split('T')[0] : null,
            lastUpdate: u.last_update ? u.last_update.split('T')[0] : "Never",
            createdAt: "Unknown",
          }))
          setUsers(mappedUsers)
        } else {
          setError("Failed to load users")
        }
      })
      .catch((e) => {
        console.error("[AdminPanel] Error loading users:", e)
        setError("Connection error")
      })
      .finally(() => setIsLoading(false))
  }

  const handleEditUser = (user: AdminUser) => {
    setEditingUser(user)
    setIsModalOpen(true)
  }

  const handleAddUser = () => {
    setEditingUser(null)
    setIsModalOpen(true)
  }

  const handleDeleteUser = (user: AdminUser) => {
    setDeletingUser(user)
    setIsDeleteModalOpen(true)
  }

  const handleConfirmDelete = () => {
    if (!deletingUser) return
    setIsSaving(true)
    sendCoreRequest("remove-user", { id: deletingUser.id })
      .then((res) => {
        if (res?.status === "ok") {
          setUsers((prev) => prev.filter((user) => user.id !== deletingUser.id))
          setIsDeleteModalOpen(false)
          setDeletingUser(null)
        } else {
          alert(res?.message ?? "Failed to delete user")
        }
      })
      .catch((e) => {
        console.error("[AdminPanel] Delete error:", e)
        alert("Error deleting user")
      })
      .finally(() => setIsSaving(false))
  }

  const handleSaveUser = (data: EditUserForm) => {
    setIsSaving(true)
    setError(null)

    if (editingUser) {
      const updatePayload: any = {
        id: editingUser.id,
        name: data.name,
        email: data.email,
        is_admin: data.role === "admin",
      }
      if (data.password) {
        updatePayload.password = data.password
      }

      sendCoreRequest("update-user", updatePayload)
        .then((res) => {
          if (res?.status === "ok") {
            setUsers((prev) =>
              prev.map((user) =>
                user.id === editingUser.id
                  ? {
                      ...user,
                      name: data.name,
                      email: data.email,
                      role: data.role,
                      lastUpdate: new Date().toISOString().split('T')[0],
                    }
                  : user,
              ),
            )
            setIsModalOpen(false)
            setEditingUser(null)
          } else {
            setError(res?.message ?? "Failed to update user")
          }
        })
        .catch((e) => {
          console.error("[AdminPanel] Update error:", e)
          setError("Error updating user")
        })
        .finally(() => setIsSaving(false))
    } else {
      sendCoreRequest("new-user", {
        name: data.name,
        email: data.email,
        password: data.password,
        is_admin: data.role === "admin",
      })
        .then((res) => {
          if (res?.status === "ok") {
            const newUser = res.data?.user
            if (newUser) {
              const mappedUser: AdminUser = {
                id: newUser.id,
                name: newUser.name,
                email: newUser.email,
                role: newUser.is_admin ? "admin" : "user",
                lastLogin: null,
                lastUpdate: new Date().toISOString().split('T')[0],
                createdAt: new Date().toISOString().split('T')[0],
              }
              setUsers((prev) => [...prev, mappedUser])
            }
            setIsModalOpen(false)
            setEditingUser(null)
          } else {
            setError(res?.message ?? "Failed to create user")
          }
        })
        .catch((e) => {
          console.error("[AdminPanel] Create error:", e)
          setError("Error creating user")
        })
        .finally(() => setIsSaving(false))
    }
  }

  return (
    <main className="relative flex min-h-[calc(100vh-4rem)] w-full flex-1 flex-col bg-[#070b10] py-5 pl-0 pr-5 text-white sm:pl-1 sm:pr-6 md:py-8 md:pl-2 md:pr-10">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(34,211,238,0.08),transparent_30%),linear-gradient(135deg,rgba(255,255,255,0.02),transparent_40%)]"
      />

      <div className="relative flex-1">
        <div className="mx-auto w-full max-w-7xl px-8 pt-2 md:pt-4">
          <div className="mb-8">
            <h1 className="text-3xl font-medium tracking-tight text-white">Admin Panel</h1>
            <p className="mt-3 text-sm text-white/50">Manage users, agents, and system settings</p>
          </div>

          <div className="space-y-8">
            {error && (
              <div className="rounded-lg bg-red-900/20 border border-red-500/30 p-4 text-red-300 text-sm">
                {error}
              </div>
            )}

            {isLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="text-white/50">Loading users...</div>
              </div>
            ) : (
              <UsersTable
                users={users}
                onEditUser={handleEditUser}
                onAddUser={handleAddUser}
                onDeleteUser={handleDeleteUser}
              />
            )}
          </div>
        </div>
      </div>

      <EditUserModal
        open={isModalOpen}
        user={editingUser}
        onClose={() => {
          setIsModalOpen(false)
          setEditingUser(null)
        }}
        onSave={handleSaveUser}
        isSaving={isSaving}
      />

      <DeleteUserModal
        open={isDeleteModalOpen}
        user={deletingUser}
        onClose={() => {
          setIsDeleteModalOpen(false)
          setDeletingUser(null)
        }}
        onConfirm={handleConfirmDelete}
        isDeleting={isSaving}
      />
    </main>
  )
}

export default AdminPanel
