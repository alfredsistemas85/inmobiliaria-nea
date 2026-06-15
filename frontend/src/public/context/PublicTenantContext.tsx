import { createContext, useContext, useEffect, useState } from 'react'

export interface PublicTenant {
  id: string
  name: string
  slug: string
  logo_url: string | null
  phone: string | null
  email: string | null
  whatsapp: string | null
  address: string | null
}

export interface PortalConfig {
  allow_contact_form: boolean
  allow_whatsapp: boolean
}

interface PublicTenantContextType {
  tenant: PublicTenant | null
  portal: PortalConfig | null
  loading: boolean
  error: string | null
}

const PublicTenantContext = createContext<PublicTenantContextType | undefined>(undefined)

export function PublicTenantProvider({ children }: { children: React.ReactNode }) {
  const [tenant, setTenant] = useState<PublicTenant | null>(null)
  const [portal, setPortal] = useState<PortalConfig | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const fetchBootstrap = async () => {
      try {
        const res = await fetch('/api/public/bootstrap')
        if (!res.ok) throw new Error('Error cargando portal')
        const data = await res.json()
        setTenant(data.tenant)
        setPortal(data.portal)
        
        // Dinamismo de titulo y favicon si existieran
        if (data.tenant) {
          document.title = data.tenant.name
        }
      } catch (err) {
        console.error(err)
        setError('No se pudo inicializar el portal')
      } finally {
        setLoading(false)
      }
    }
    
    fetchBootstrap()
  }, [])

  return (
    <PublicTenantContext.Provider value={{ tenant, portal, loading, error }}>
      {children}
    </PublicTenantContext.Provider>
  )
}

export function usePublicTenant() {
  const context = useContext(PublicTenantContext)
  if (!context) throw new Error('usePublicTenant debe ser usado dentro de PublicTenantProvider')
  return context
}
