import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { connectCore, sendCoreRequest, logout as wsLogout } from "@/lib/ws";

export type UserData = {
  id?: number;
  name?: string;
  email?: string;
  is_admin?: boolean;
  last_update?: string | null;
};

type UserContextType = {
  user: UserData | null;
  token: string | null;
  isAuthenticated: boolean | null; // null = loading, true/false
  login: (token: string, user: UserData) => void;
  logout: () => void;
  updateUser: (userData: UserData) => void;
};

const UserContext = createContext<UserContextType | undefined>(undefined);

export function UserProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [user, setUser] = useState<UserData | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null);

  useEffect(() => {
    let isMounted = true;

    const storedToken = (() => {
      try { return localStorage.getItem("sm_token"); } catch { return null; }
    })();

    const storedUser = (() => {
      try {
        const data = localStorage.getItem("sm_userData");
        return data ? JSON.parse(data) : null;
      } catch { return null; }
    })();

    if (storedToken) {
      setToken(storedToken);
      if (storedUser) setUser(storedUser);

      connectCore();
      sendCoreRequest("authenticate", { token: storedToken })
        .then((res) => {
          if (!isMounted) return;
          if (res?.status === "ok") {
            setIsAuthenticated(true);

            if (res.data?.user) {
              const updatedUser = {
                id: res.data.user.id,
                name: res.data.user.name,
                email: res.data.user.email,
                is_admin: res.data.user.is_admin,
                last_update: res.data.user.last_update,
              };

              setUser(updatedUser);
              try { localStorage.setItem("sm_userData", JSON.stringify(updatedUser)); } catch {}
            }
          } else {
            setToken(null);
            setUser(null);
            setIsAuthenticated(false);
            localStorage.removeItem("sm_token");
            localStorage.removeItem("sm_userData");
          }
        })
        .catch(() => {
          if (!isMounted) return;
          setToken(null);
          setUser(null);
          setIsAuthenticated(false);
          localStorage.removeItem("sm_token");
          localStorage.removeItem("sm_userData");
        });
    } else {
      setIsAuthenticated(false);
    }

    return () => { isMounted = false; };
  }, []);

  const login = useCallback((newToken: string, newUser: UserData) => {
    try { localStorage.setItem("sm_token", newToken); } catch {}
    const userToSave = {
      id: newUser.id,
      name: newUser.name,
      email: newUser.email,
      is_admin: newUser.is_admin,
      last_update: newUser.last_update,
    };
    try { localStorage.setItem("sm_userData", JSON.stringify(userToSave)); } catch {}
    setToken(newToken);
    setUser(userToSave);
    setIsAuthenticated(true);
    connectCore();
    sendCoreRequest("authenticate", { token: newToken })
      .then((res) => {
        if (res?.status !== "ok") {
          setToken(null);
          setUser(null);
          setIsAuthenticated(false);
          localStorage.removeItem("sm_token");
          localStorage.removeItem("sm_userData");
        }
      })
      .catch(() => {
        setToken(null);
        setUser(null);
        setIsAuthenticated(false);
        localStorage.removeItem("sm_token");
        localStorage.removeItem("sm_userData");
      });
  }, []);

  const logout = useCallback(() => {
    wsLogout();
    setToken(null);
    setUser(null);
    setIsAuthenticated(false);
    localStorage.removeItem("sm_token");
    localStorage.removeItem("sm_userData");
  }, []);

  const updateUser = useCallback((updatedUser: UserData) => {
    setUser(updatedUser);
    try { localStorage.setItem("sm_userData", JSON.stringify(updatedUser)); } catch {}
  }, []);

  const value: UserContextType = {
    user,
    token,
    isAuthenticated,
    login,
    logout,
    updateUser,
  };

  return <UserContext.Provider value={value}>{children}</UserContext.Provider>;
}

export function useUser() {
  const context = useContext(UserContext);
  if (context === undefined) {
    throw new Error("useUser must be used within a UserProvider");
  }
  return context;
}
