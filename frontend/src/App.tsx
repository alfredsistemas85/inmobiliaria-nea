import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { DashboardLayout } from '@/layouts/DashboardLayout'

// Pages
import Login from '@/pages/Login'
import Dashboard from '@/pages/Dashboard'
import Properties from '@/pages/Properties'
import PropertyDetail from '@/pages/PropertyDetail'
import PropertyForm from '@/pages/PropertyForm'
import Clients from '@/pages/Clients'
import Leads from '@/pages/Leads'
import Appointments from '@/pages/Appointments'
import WhatsApp from '@/pages/WhatsApp'
import Reports from '@/pages/Reports'
import Users from '@/pages/Users'
import Settings from '@/pages/Settings'

/**
 * Guard de autenticación.
 * Si no hay token en localStorage, redirige al login SIN montar DashboardLayout.
 * Esto corta el loop: 401 → redirect a /login → guard verifica token → ok o vuelve a login.
 */
function RequireAuth({ children }: { children: React.ReactNode }) {
  const token = localStorage.getItem('token')
  if (!token) {
    return <Navigate to="/login" replace />
  }
  return <>{children}</>
}

import PublicLayout from '@/public/PublicLayout'
import HomePage from '@/public/pages/HomePage'
import PropertyListPage from '@/public/pages/PropertyListPage'
import PropertyDetailPage from '@/public/pages/PropertyDetailPage'
import ContactPage from '@/public/pages/ContactPage'

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* PUBLIC PORTAL */}
        <Route element={<PublicLayout />}>
          <Route path="/" element={<HomePage />} />
          <Route path="/buscar" element={<PropertyListPage />} />
          <Route path="/propiedad/:id" element={<PropertyDetailPage />} />
          <Route path="/contacto" element={<ContactPage />} />
        </Route>

        <Route path="/login" element={<Login />} />

        {/* CRM ADMINISTRATIVO (rutas directas protegidas) */}
        <Route
          path="/"
          element={
            <RequireAuth>
              <DashboardLayout />
            </RequireAuth>
          }
        >
          {/* No definimos index para CRM aquí para no pisar el HomePage, el login redirige directo a /dashboard */}
          <Route path="dashboard"          element={<Dashboard />} />
          <Route path="properties"         element={<Properties />} />
          <Route path="properties/new"     element={<PropertyForm />} />
          <Route path="properties/:id"     element={<PropertyDetail />} />
          <Route path="properties/:id/edit" element={<PropertyForm />} />
          <Route path="clients"            element={<Clients />} />
          <Route path="leads"              element={<Leads />} />
          <Route path="appointments"       element={<Appointments />} />
          <Route path="whatsapp"           element={<WhatsApp />} />
          <Route path="reports"            element={<Reports />} />
          <Route path="users"              element={<Users />} />
          <Route path="settings"           element={<Settings />} />
        </Route>

        {/* Fallback */}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
