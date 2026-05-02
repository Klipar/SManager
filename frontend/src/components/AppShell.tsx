import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useUser } from "@/contexts/UserContext";
import { AppProvider } from "@/contexts/AppContext";
import { HomePage } from "@/pages/home/HomePage";
import { LoginPage } from "@/pages/auth/LoginPage";

function AppShell() {
  const { isAuthenticated, isLoggingOut, login } = useUser();
  const navigate = useNavigate();

  useEffect(() => {
    if (isAuthenticated === false) {
      navigate("/", { replace: true });
    }
  }, [isAuthenticated, navigate]);

  if (isAuthenticated === null || isLoggingOut) return null;
  if (!isAuthenticated) return <LoginPage onLogin={login} />;

  return (
    <AppProvider>
      <HomePage />
    </AppProvider>
  );
}

export default AppShell
