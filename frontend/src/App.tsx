import { BrowserRouter } from "react-router-dom"
import { UserProvider } from "@/contexts/UserContext"
import AppShell from "@/components/AppShell"

function App() {
  return (
    <BrowserRouter>
      <UserProvider>
        <AppShell />
      </UserProvider>
    </BrowserRouter>
  )
}

export default App
