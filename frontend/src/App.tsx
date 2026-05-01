import { BrowserRouter } from "react-router-dom";
import { UserProvider, useUser } from "@/contexts/UserContext";
import { AppProvider } from "@/contexts/AppContext";
import { HomePage } from "./features/home/home-page";
import { LoginPage } from "./features/auth/login-page";

function AppContent() {
  const { isAuthenticated, login } = useUser();

  if (isAuthenticated === null) {
    return <div className="min-h-screen w-full bg-[#070b10]" />;
  }
  if (!isAuthenticated) {
    return <LoginPage onLogin={login} />;
  }
  return (
    <AppProvider>
      <HomePage />
    </AppProvider>
  );
}

function App() {
  return (
    <BrowserRouter>
      <UserProvider>
        <AppContent />
      </UserProvider>
    </BrowserRouter>
  );
}

export default App
