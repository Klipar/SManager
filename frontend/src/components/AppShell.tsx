import { useUser } from "@/contexts/UserContext"
import { AppProvider } from "@/contexts/AppContext"
import { HomePage } from "@/features/home/home-page"
import { LoginPage } from "@/features/auth/login-page"

function AppShell() {
  const { isAuthenticated, login } = useUser()

  if (isAuthenticated === null) {
    return <div className="min-h-screen w-full bg-[#070b10]" />
  }

  if (!isAuthenticated) {
    return <LoginPage onLogin={login} />
  }

  return (
    <AppProvider>
      <HomePage />
    </AppProvider>
  )
}

export default AppShell
