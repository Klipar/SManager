import { Navigate, Routes, Route } from "react-router-dom";
import { Layout } from "./Layout";
import { Dashboard } from "./Dashboard";
import AccountPanel from "../account/AccountPanel";
import AdminPanel from "../admin/AdminPanel";
import { useUser } from "@/contexts/UserContext";

function HomePage() {
  const { user } = useUser();
  const isAdmin = Boolean(user?.is_admin);

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Dashboard />} />
        <Route path="account" element={<AccountPanel />} />
        <Route path="admin" element={isAdmin ? <AdminPanel /> : <Navigate to="/" replace />} />
      </Route>
    </Routes>
  );
}

export { HomePage }
