import { Navigate, Routes, Route, useParams } from "react-router-dom";
import { Layout } from "./Layout";
import { Dashboard } from "./Dashboard";
import CreateTaskPanel from "../task/CreateTaskPanel";
import AccountPanel from "../account/AccountPanel";
import AdminPanel from "../admin/AdminPanel";
import { useApp } from "@/contexts/AppContext";
import { useUser } from "@/contexts/UserContext";

function CreateTaskPage() {
  const { agentId } = useParams();
  const { agents } = useApp();
  const agent = agentId ? (agents.find(a => a.id === agentId) ?? null) : null;
  return <CreateTaskPanel agent={agent} />;
}

function HomePage() {
  const { user } = useUser();
  const isAdmin = Boolean(user?.is_admin);

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Dashboard />} />
        <Route path="task/new/:agentId?" element={<CreateTaskPage />} />
        <Route path="account" element={<AccountPanel />} />
        <Route path="admin" element={isAdmin ? <AdminPanel /> : <Navigate to="/" replace />} />
      </Route>
    </Routes>
  );
}

export { HomePage }
