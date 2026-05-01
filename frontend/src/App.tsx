import { useEffect, useState } from "react"
import { HomePage } from "./features/home/home-page"
import { LoginPage } from "./features/auth/login-page"
import { connectCore, sendCoreRequest } from "./lib/ws"

type UserData = { id?: number; name?: string; email?: string; is_admin?: boolean; last_update?: string | null }

function App() {
  const [authenticated, setAuthenticated] = useState<boolean | null>(null)
  const [userData, setUserData] = useState<UserData | null>(null)

  useEffect(() => {
    let isMounted = true

    const token = (() => {
      try { return localStorage.getItem("sm_token") }
      catch { return null }
    })()

    // Restore userData from localStorage on mount
    const savedUserData = (() => {
      try {
        const data = localStorage.getItem("sm_userData")
        return data ? JSON.parse(data) : null
      } catch { return null }
    })()
    if (savedUserData && isMounted) {
      setUserData(savedUserData)
    }

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
            localStorage.removeItem("sm_userData")
          }
        })
        .catch((err) => {
          if (!isMounted) return
          console.error("[App] Auth error:", err)
          setAuthenticated(false)
          localStorage.removeItem("sm_token")
          localStorage.removeItem("sm_userData")
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
          if (_user && typeof _user === 'object') {
            const newUserData = {
              id: (_user as any).id,
              name: (_user as any).name,
              email: (_user as any).email,
              is_admin: (_user as any).is_admin,
              last_update: (_user as any).last_update,
            }
            setUserData(newUserData)
            // Save userData to localStorage for recovery on page reload
            try { localStorage.setItem('sm_userData', JSON.stringify(newUserData)) } catch {}
          }
          connectCore()
          sendCoreRequest('authenticate', { token })
            .then((res) => {
              console.log('[App] post-login authenticate response:', res)
              if (res && res.status === 'ok') {
                setAuthenticated(true)
              } else {
                setAuthenticated(false)
                localStorage.removeItem('sm_token')
                localStorage.removeItem('sm_userData')
              }
            })
            .catch((e) => {
              console.error('[App] authenticate after login failed', e)
              setAuthenticated(false)
              localStorage.removeItem('sm_token')
              localStorage.removeItem('sm_userData')
            })
        }}
      />
    )
  }

  return <HomePage userData={userData} onUpdateUser={setUserData} />
}

export default App
