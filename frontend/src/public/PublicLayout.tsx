import { Outlet } from 'react-router-dom'
import { PublicTenantProvider, usePublicTenant } from './context/PublicTenantContext'

function LayoutContent() {
  const { tenant, loading, error } = usePublicTenant()

  if (loading) return <div className="flex h-screen items-center justify-center bg-gray-50 text-gray-400">Cargando portal...</div>
  
  if (error || !tenant) return (
    <div className="flex h-screen items-center justify-center bg-red-50 text-red-500 flex-col">
      <h1 className="text-2xl font-bold mb-2">Portal no encontrado</h1>
      <p>No se pudo resolver el tenant para este dominio.</p>
    </div>
  )

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-sans">
      <header className="bg-white shadow-sm sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
          <div className="flex items-center gap-4 cursor-pointer" onClick={() => window.location.href = '/'}>
            {tenant.logo_url ? (
              <img src={tenant.logo_url} alt={tenant.name} className="h-10 w-auto object-contain" />
            ) : (
              <div className="h-10 w-10 bg-indigo-600 rounded-md flex items-center justify-center text-white font-bold text-xl">
                {tenant.name.charAt(0)}
              </div>
            )}
            <span className="font-bold text-xl text-gray-900 tracking-tight">{tenant.name}</span>
          </div>
          <nav className="hidden md:flex gap-8">
            <a href="/" className="text-gray-600 hover:text-indigo-600 font-medium transition-colors">Inicio</a>
            <a href="/buscar" className="text-gray-600 hover:text-indigo-600 font-medium transition-colors">Propiedades</a>
            <a href="/contacto" className="text-gray-600 hover:text-indigo-600 font-medium transition-colors">Contacto</a>
          </nav>
        </div>
      </header>

      <main className="flex-grow">
        <Outlet />
      </main>

      <footer className="bg-gray-900 text-white py-12 mt-auto">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row justify-between items-center gap-6">
          <div>
            <h2 className="text-2xl font-bold">{tenant.name}</h2>
            <p className="text-gray-400 mt-2">{tenant.address}</p>
          </div>
          <div className="flex gap-6">
             {tenant.whatsapp && <a href={`https://wa.me/${tenant.whatsapp.replace(/\D/g,'')}`} target="_blank" rel="noreferrer" className="text-gray-400 hover:text-green-400">WhatsApp</a>}
             {tenant.email && <a href={`mailto:${tenant.email}`} className="text-gray-400 hover:text-white">Email</a>}
          </div>
        </div>
      </footer>
    </div>
  )
}

export default function PublicLayout() {
  return (
    <PublicTenantProvider>
      <LayoutContent />
    </PublicTenantProvider>
  )
}
