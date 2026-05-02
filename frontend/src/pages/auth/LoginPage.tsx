import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { LoginForm } from "./LoginForm"
import type { UserData } from "@/types"

type LoginPageProps = {
  onLogin: (token: string, user: UserData) => void
}

function LoginPage({ onLogin }: LoginPageProps) {
  return (
    <main className="relative min-h-screen overflow-hidden bg-[#0b0f14] text-white">
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.08),_transparent_34%),radial-gradient(circle_at_bottom,_rgba(255,255,255,0.05),_transparent_28%)]"
      />
      <section className="relative flex min-h-screen items-center justify-center px-4 py-10 sm:px-6 lg:px-8">
        <div className="w-full max-w-md text-center">
          <div className="mb-8 space-y-3">
            <h1 className="text-4xl font-semibold tracking-[0.18em] text-white sm:text-5xl">
              SManager
            </h1>
            <p className="text-sm text-white/50">
              Sign in to continue to your workspace.
            </p>
          </div>

          <Card className="mx-auto w-full max-w-sm border-white/10 bg-white/[0.03] text-left">
            <CardHeader className="pb-0">
              <CardTitle className="sr-only">Login form</CardTitle>
              <CardDescription id="login-help">
                Enter your credentials to access the application.
              </CardDescription>
            </CardHeader>
            <CardContent className="pt-6">
              <LoginForm onSuccess={(token, user) => onLogin(token, user)} />
            </CardContent>
          </Card>
        </div>
      </section>
    </main>
  )
}

export { LoginPage }
