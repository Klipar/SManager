import { Routes, Route, useParams } from "react-router-dom";
import { Layout } from "./Layout";
import { Dashboard } from "./Dashboard";
import CreateTaskPanel from "../task/CreateTaskPanel";
import AccountPanel from "../account/AccountPanel";
import AdminPanel from "../admin/AdminPanel";
import { useApp } from "@/contexts/AppContext";

function CreateTaskPage() {
  const { agentId } = useParams();
  const { agents } = useApp();
  const agent = agentId ? (agents.find(a => a.id === agentId) ?? null) : null;
  return <CreateTaskPanel agent={agent} />;
}

function HomePage() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Dashboard />} />
        <Route path="task/new/:agentId?" element={<CreateTaskPage />} />
        <Route path="account" element={<AccountPanel />} />
        <Route path="admin" element={<AdminPanel />} />
      </Route>
    </Routes>
  );
}

export { HomePage }
