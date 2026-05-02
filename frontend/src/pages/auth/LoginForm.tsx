import { useState, type FormEvent } from "react"
import { sendCoreRequest } from "@/lib/ws"
import { Button } from "@/components/ui/Button"
import { Input } from "@/components/ui/Input"
import { Label } from "@/components/ui/Label"
import type { UserData } from "@/types"

type LoginFormState = {
  username: string
  password: string
}

const initialFormState: LoginFormState = {
  username: "",
  password: "",
}

type LoginFormProps = {
  onSuccess?: (token: string, user: UserData) => void
}

function LoginForm({ onSuccess }: LoginFormProps) {
  const [formState, setFormState] = useState<LoginFormState>(initialFormState)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    setIsSubmitting(true)
    setError(null)

    try {
      const res = await sendCoreRequest("login", {
        login: formState.username,
        password: formState.password,
      })

      const token = res?.data?.auth?.token
      const user = res?.data?.auth?.user

      if (res?.status === "ok" && token) {
        try { localStorage.setItem("sm_token", token) } catch {}
        onSuccess?.(token, user as UserData)
      } else {
        setError(res?.message ?? "Login failed")
      }
    } catch (e) {
      console.error("[LoginForm] error:", e)
      setError("Connection error, please try again")
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <form className="space-y-5" onSubmit={handleSubmit} noValidate>
      <div>
        <Label htmlFor="username">Username</Label>
        <Input
          id="username"
          name="username"
          autoComplete="username"
          placeholder="login"
          value={formState.username}
          onChange={(event) =>
            setFormState((current) => ({ ...current, username: event.target.value }))
          }
        />
      </div>
      <div>
        <Label htmlFor="password">Password</Label>
        <Input
          id="password"
          name="password"
          type="password"
          autoComplete="current-password"
          placeholder="password"
          value={formState.password}
          onChange={(event) =>
            setFormState((current) => ({ ...current, password: event.target.value }))
          }
        />
      </div>

      {error ? (
        <p className="text-sm text-red-400">{error}</p>
      ) : null}

      <Button
        type="submit"
        className="h-12 w-full rounded-xl text-base font-medium tracking-wide"
        disabled={isSubmitting}
      >
        {isSubmitting ? "Signing in..." : "LOGIN"}
      </Button>
    </form>
  )
}

export { LoginForm }
