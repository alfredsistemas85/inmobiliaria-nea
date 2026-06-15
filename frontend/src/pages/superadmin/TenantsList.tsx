import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { Plus, Search, Building2, MoreHorizontal } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'

export default function SuperAdminTenants() {
  const [tenants, setTenants] = useState<any[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // Mock fetch for now, will connect to API later
    setTimeout(() => {
      setTenants([
        { id: '1', business_name: 'Inmobiliaria Central', cuit: '30-71234567-8', status: 'ACTIVE', created_at: '2024-01-10T10:00:00Z' },
        { id: '2', business_name: 'Propiedades del Norte', cuit: '33-65432109-9', status: 'PENDING', created_at: '2024-06-15T14:30:00Z' },
        { id: '3', business_name: 'Sur Bienes Raíces', cuit: '30-11223344-5', status: 'SUSPENDED', created_at: '2023-11-05T09:15:00Z' }
      ])
      setLoading(false)
    }, 500)
  }, [])

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-foreground">Inmobiliarias</h1>
          <p className="text-muted-foreground">Gestión de todos los tenants del sistema.</p>
        </div>
        <Button className="bg-purple-600 hover:bg-purple-700">
          <Plus className="mr-2 h-4 w-4" />
          Nueva Inmobiliaria
        </Button>
      </div>

      <Card>
        <div className="p-4 border-b border-border flex flex-col sm:flex-row gap-4 justify-between">
          <div className="relative max-w-sm w-full">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input type="text" placeholder="Buscar por nombre o CUIT..." className="pl-9" />
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm">Todas</Button>
            <Button variant="ghost" size="sm" className="text-green-600">Activas</Button>
            <Button variant="ghost" size="sm" className="text-amber-600">Pendientes</Button>
          </div>
        </div>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  <th className="px-6 py-3 font-medium">Inmobiliaria</th>
                  <th className="px-6 py-3 font-medium">CUIT</th>
                  <th className="px-6 py-3 font-medium">Estado</th>
                  <th className="px-6 py-3 font-medium">Fecha Alta</th>
                  <th className="px-6 py-3 text-right font-medium">Acciones</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {loading ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">
                      Cargando inmobiliarias...
                    </td>
                  </tr>
                ) : (
                  tenants.map(tenant => (
                    <tr key={tenant.id} className="hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4">
                        <div className="flex items-center gap-3">
                          <div className="h-8 w-8 rounded bg-purple-100 flex items-center justify-center text-purple-600">
                            <Building2 className="h-4 w-4" />
                          </div>
                          <span className="font-medium text-foreground">{tenant.business_name}</span>
                        </div>
                      </td>
                      <td className="px-6 py-4 text-muted-foreground">{tenant.cuit}</td>
                      <td className="px-6 py-4">
                        <Badge variant="outline" className={cn(
                          tenant.status === 'ACTIVE' && "border-green-500 text-green-500",
                          tenant.status === 'PENDING' && "border-amber-500 text-amber-500",
                          tenant.status === 'SUSPENDED' && "border-red-500 text-red-500",
                        )}>
                          {tenant.status}
                        </Badge>
                      </td>
                      <td className="px-6 py-4 text-muted-foreground">
                        {new Date(tenant.created_at).toLocaleDateString('es-AR')}
                      </td>
                      <td className="px-6 py-4 text-right">
                        <Link to={`/superadmin/tenants/${tenant.id}`}>
                          <Button variant="ghost" size="sm">
                            <MoreHorizontal className="h-4 w-4" />
                          </Button>
                        </Link>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
