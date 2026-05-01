import { useEffect, useState } from "react"
import { HomePage } from "./features/home/home-page"
import { LoginPage } from "./features/auth/login-page"
import { connectCore, sendCoreRequest } from "./lib/ws"

function App() {
  const [authenticated, setAuthenticated] = useState<boolean | null>(null)

  useEffect(() => {
    let isMounted = true

    const token = (() => {
      try { return localStorage.getItem("sm_token") }
      catch { return null }
    })()

    if (token) {
      connectCore()
      sendCoreRequest("authenticate", { token })
        .then((res) => {
          if (!isMounted) return
          if (res?.status === "ok") {
            setAuthenticated(true)
          } else {
            setAuthenticated(false)
            localStorage.removeItem("sm_token")
          }
        })
        .catch((err) => {
          if (!isMounted) return
          console.error("[App] Auth error:", err)
          setAuthenticated(false)
          localStorage.removeItem("sm_token")
        })
    } else {
      setAuthenticated(false)
    }

    return () => {
      isMounted = false
    }
  }, [])

  if (authenticated === null) return <div className="min-h-screen w-full bg-[#070b10]" />
  if (!authenticated) {
    return (
      <LoginPage
        onLogin={(token, _user) => {
          try { localStorage.setItem('sm_token', token) } catch {}
          connectCore()
          sendCoreRequest('authenticate', { token })
            .then((res) => {
              console.log('[App] post-login authenticate response:', res)
              if (res && res.status === 'ok') {
                setAuthenticated(true)
              } else {
                setAuthenticated(false)
                localStorage.removeItem('sm_token')
              }
            })
            .catch((e) => {
              console.error('[App] authenticate after login failed', e)
              setAuthenticated(false)
              localStorage.removeItem('sm_token')
            })
        }}
      />
    )
  }

  return <HomePage />
}

export default App
