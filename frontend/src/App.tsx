import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { DashboardLayout } from '@/layouts/DashboardLayout'

// Pages
import Login from '@/pages/Login'
import Dashboard from '@/pages/Dashboard'
import Properties from '@/pages/Properties'
import PropertyDetail from '@/pages/PropertyDetail'
import Clients from '@/pages/Clients'
import Leads from '@/pages/Leads'
import Appointments from '@/pages/Appointments'
import WhatsApp from '@/pages/WhatsApp'
import Reports from '@/pages/Reports'
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

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />

        {/* Todas las rutas protegidas pasan por RequireAuth */}
        <Route
          path="/"
          element={
            <RequireAuth>
              <DashboardLayout />
            </RequireAuth>
          }
        >
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard"          element={<Dashboard />} />
          <Route path="properties"         element={<Properties />} />
          <Route path="properties/:id"     element={<PropertyDetail />} />
          <Route path="clients"            element={<Clients />} />
          <Route path="leads"              element={<Leads />} />
          <Route path="appointments"       element={<Appointments />} />
          <Route path="whatsapp"           element={<WhatsApp />} />
          <Route path="reports"            element={<Reports />} />
          <Route path="settings"           element={<Settings />} />
        </Route>

        {/* Fallback */}
        <Route path="*" element={<Navigate to="/dashboard" replace />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
