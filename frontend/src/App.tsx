import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { DashboardLayout } from '@/layouts/DashboardLayout'
import { SuperAdminLayout } from '@/layouts/SuperAdminLayout'

import { Suspense, lazy } from 'react'

// Pages (Lazy loaded)
const Login = lazy(() => import('@/pages/Login'))
const Dashboard = lazy(() => import('@/pages/Dashboard'))
const Properties = lazy(() => import('@/pages/Properties'))
const PropertyDetail = lazy(() => import('@/pages/PropertyDetail'))
const PropertyForm = lazy(() => import('@/pages/PropertyForm'))
const Clients = lazy(() => import('@/pages/Clients'))
const Leads = lazy(() => import('@/pages/Leads'))
const Appointments = lazy(() => import('@/pages/Appointments'))
const WhatsApp = lazy(() => import('@/pages/WhatsApp'))
const Reports = lazy(() => import('@/pages/Reports'))
const Users = lazy(() => import('@/pages/Users'))
const Settings = lazy(() => import('@/pages/Settings'))
const Contracts = lazy(() => import('@/pages/Contracts'))
const NewContract = lazy(() => import('@/pages/NewContract'))
const Financials = lazy(() => import('@/pages/Financials'))
const Documents = lazy(() => import('@/pages/Documents'))
const ContractSignaturePage = lazy(() => import('@/pages/signatures/ContractSignaturePage'))

// SuperAdmin Pages (Lazy loaded)
const SuperAdminDashboard = lazy(() => import('@/pages/superadmin/Dashboard'))
const SuperAdminTenants = lazy(() => import('@/pages/superadmin/TenantsList'))
const SuperAdminTenantDetail = lazy(() => import('@/pages/superadmin/TenantDetail'))
const SuperAdminSupport = lazy(() => import('@/pages/superadmin/Support'))
const SuperAdminMonitoring = lazy(() => import('@/pages/superadmin/Monitoring'))
const SuperAdminSettings = lazy(() => import('@/pages/superadmin/Settings'))

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



const VerifyEmail = lazy(() => import('@/pages/VerifyEmail'))
const Onboarding = lazy(() => import('@/pages/Onboarding'))
import { ErrorBoundary } from '@/components/ErrorBoundary'

function App() {
  return (
    <ErrorBoundary>
      <BrowserRouter>
        <Suspense fallback={<div className="flex h-screen w-screen items-center justify-center"><div className="h-8 w-8 rounded-full border-2 border-primary border-t-transparent animate-spin" /></div>}>
          <Routes>
            {/* Root Redirect */}
          <Route path="/" element={<Navigate to="/login" replace />} />

          <Route path="/login" element={<Login />} />
        <Route path="/verify-email" element={<VerifyEmail />} />
        <Route path="/onboarding" element={<Onboarding />} />

        {/* Rutas Públicas de Firma Electrónica */}
        <Route path="/s/:token" element={<ContractSignaturePage />} />

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
          <Route path="/contracts/new"      element={<NewContract />} />
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
        </Suspense>
      </BrowserRouter>
    </ErrorBoundary>
  )
}

export default App
