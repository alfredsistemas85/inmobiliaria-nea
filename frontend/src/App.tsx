import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { DashboardLayout } from '@/layouts/DashboardLayout'
import { SuperAdminLayout } from '@/layouts/SuperAdminLayout'

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
import Contracts from '@/pages/Contracts'
import Financials from '@/pages/Financials'
import Documents from '@/pages/Documents'

// SuperAdmin Pages
import SuperAdminDashboard from '@/pages/superadmin/Dashboard'
import SuperAdminTenants from '@/pages/superadmin/TenantsList'
import SuperAdminTenantDetail from '@/pages/superadmin/TenantDetail'
import SuperAdminSupport from '@/pages/superadmin/Support'
import SuperAdminMonitoring from '@/pages/superadmin/Monitoring'
import SuperAdminSettings from '@/pages/superadmin/Settings'

/**
 * Guard de autenticación.
 * Si no hay token en localStorage, redirige al login SIN montar DashboardLayout.
 * Esto corta el loop: 401 → redirect a /login → guard verifica token → ok o vuelve a login.
 */
function RequireAuth({ children }: { children: React.ReactNode }) {
  const token = localStorage.getItem('token')
  const userStr = localStorage.getItem('user')
  const user = userStr ? JSON.parse(userStr) : null

  if (!token) {
    return <Navigate to="/login" replace />
  }

  if (user?.role === 'super_admin') {
    return <Navigate to="/superadmin" replace />
  }

  return <>{children}</>
}

function RequireSuperAdmin({ children }: { children: React.ReactNode }) {
  const token = localStorage.getItem('token')
  const userStr = localStorage.getItem('user')
  const user = userStr ? JSON.parse(userStr) : null

  if (!token || user?.role !== 'super_admin') {
    return <Navigate to="/dashboard" replace />
  }
  return <>{children}</>
}



import VerifyEmail from '@/pages/VerifyEmail'
import Onboarding from '@/pages/Onboarding'
import { ErrorBoundary } from '@/components/ErrorBoundary'

function App() {
  return (
    <ErrorBoundary>
      <BrowserRouter>
        <Routes>
          {/* Root Redirect */}
          <Route path="/" element={<Navigate to="/login" replace />} />

          <Route path="/login" element={<Login />} />
        <Route path="/verify-email" element={<VerifyEmail />} />
        <Route path="/onboarding" element={<Onboarding />} />

        {/* CRM ADMINISTRATIVO (rutas directas protegidas) */}
        <Route
          element={
            <RequireAuth>
              <DashboardLayout />
            </RequireAuth>
          }
        >
          {/* No definimos index para CRM aquí para no pisar el HomePage, el login redirige directo a /dashboard */}
          <Route path="/dashboard"          element={<Dashboard />} />
          <Route path="/properties"         element={<Properties />} />
          <Route path="/properties/new"     element={<PropertyForm />} />
          <Route path="/properties/:id"     element={<PropertyDetail />} />
          <Route path="/properties/:id/edit" element={<PropertyForm />} />
          <Route path="/clients"            element={<Clients />} />
          <Route path="/leads"              element={<Leads />} />
          <Route path="/appointments"       element={<Appointments />} />
          <Route path="/whatsapp"           element={<WhatsApp />} />
          <Route path="/reports"            element={<Reports />} />
          <Route path="/users"              element={<Users />} />
          <Route path="/settings"           element={<Settings />} />
          <Route path="/contracts"          element={<Contracts />} />
          <Route path="/financials"         element={<Financials />} />
          <Route path="/documents"          element={<Documents />} />
        </Route>

        {/* SUPERADMIN PANEL */}
        <Route
          path="/superadmin"
          element={
            <RequireSuperAdmin>
              <SuperAdminLayout />
            </RequireSuperAdmin>
          }
        >
          <Route index element={<SuperAdminDashboard />} />
          <Route path="tenants" element={<SuperAdminTenants />} />
          <Route path="tenants/:id" element={<SuperAdminTenantDetail />} />
          <Route path="monitoring" element={<SuperAdminMonitoring />} />
          <Route path="support" element={<SuperAdminSupport />} />
          <Route path="settings" element={<SuperAdminSettings />} />
        </Route>

        {/* Fallback */}
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
    </ErrorBoundary>
  )
}

export default App
