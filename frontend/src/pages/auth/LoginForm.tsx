import { useState } from "react"
import { sendCoreRequest } from "@/lib/ws"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
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
  const [usernameError, setUsernameError] = useState<string | null>(null)
  const [passwordError, setPasswordError] = useState<string | null>(null)
  const [submitError, setSubmitError] = useState<string | null>(null)

  const validateForm = () => {
    const nextUsernameError = !formState.username.trim() ? "Username is required" : null
    const nextPasswordError = !formState.password.trim() ? "Password is required" : null

    setUsernameError(nextUsernameError)
    setPasswordError(nextPasswordError)

    return !nextUsernameError && !nextPasswordError
  }

  const handleSubmit = async (event: React.SyntheticEvent<HTMLFormElement, SubmitEvent>) => {
    event.preventDefault()

    const isValid = validateForm()
    if (!isValid) {
      return
    }

    setIsSubmitting(true)
    setSubmitError(null)

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
        setSubmitError(res?.message ?? "Login failed")
      }
    } catch (e) {
      console.error("[LoginForm] error:", e)
      setSubmitError("Connection error, please try again")
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
          onChange={(event) => {
            setFormState((current) => ({ ...current, username: event.target.value }))
            if (usernameError) setUsernameError(null)
            if (submitError) setSubmitError(null)
          }}
        />
        {usernameError ? <p className="mt-2 text-sm text-rose-400">{usernameError}</p> : null}
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
          onChange={(event) => {
            setFormState((current) => ({ ...current, password: event.target.value }))
            if (passwordError) setPasswordError(null)
            if (submitError) setSubmitError(null)
          }}
        />
        {passwordError ? <p className="mt-2 text-sm text-rose-400">{passwordError}</p> : null}
        {submitError ? <p className="mt-2 text-sm text-rose-400">{submitError}</p> : null}
      </div>

      <Button
        type="submit"
        className="h-12 w-full rounded-xl text-base font-medium tracking-wide"
        disabled={isSubmitting || !formState.username.trim() || !formState.password.trim()}
      >
        {isSubmitting ? "Signing in..." : "LOGIN"}
      </Button>
    </form>
  )
}

export { LoginForm }
