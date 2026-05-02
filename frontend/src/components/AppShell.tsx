import { useUser } from "@/contexts/UserContext"
import { AppProvider } from "@/contexts/AppContext"
import { HomePage } from "@/pages/home/HomePage"
import { LoginPage } from "@/pages/auth/LoginPage"

function AppShell() {
  const { isAuthenticated, isLoggingOut, login } = useUser()

  if (isAuthenticated === null || isLoggingOut) {
    return (
      <div className="min-h-screen w-full bg-[#070b10] flex items-center justify-center">
        <div className="text-white/70 text-lg">
          {isLoggingOut ? "Logging out..." : "Loading..."}
        </div>
      </div>
    )
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
