import { useState, type FormEvent } from "react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

type LoginFormState = {
  username: string
  password: string
}

const initialFormState: LoginFormState = {
  username: "",
  password: "",
}

function LoginForm() {
  const [formState, setFormState] = useState<LoginFormState>(initialFormState)
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    setIsSubmitting(true)

    try {
      await new Promise((resolve) => window.setTimeout(resolve, 700))
      console.info("Login submitted", {
        username: formState.username,
        passwordLength: formState.password.length,
      })
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
            setFormState((current) => ({
              ...current,
              username: event.target.value,
            }))
          }
          aria-describedby="login-help"
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
            setFormState((current) => ({
              ...current,
              password: event.target.value,
            }))
          }
        />
      </div>

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
