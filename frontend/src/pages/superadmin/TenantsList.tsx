import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { Plus, Search, Building2, MoreHorizontal, Loader2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Modal } from '@/components/ui/modal'
import { superadminService } from '@/services/superadmin'
import { cn } from '@/lib/utils'

export default function SuperAdminTenants() {
  const [tenants, setTenants] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [searchTerm, setSearchTerm] = useState('')
  const [filter, setFilter] = useState('ALL')
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [newTenant, setNewTenant] = useState({
    business_name: '', cuit: '', dni_responsable: '', first_name: '', last_name: '', admin_email: '', phone: ''
  })
  const [toastMessage, setToastMessage] = useState<{msg: string, type: 'error' | 'success'} | null>(null)

  const showToast = (msg: string, type: 'error' | 'success' = 'success') => {
    setToastMessage({ msg, type })
    setTimeout(() => setToastMessage(null), 5000)
  }

  useEffect(() => {
    loadTenants()
  }, [])

  const loadTenants = async () => {
    try {
      setLoading(true)
      const data = await superadminService.getTenants()
      setTenants(data || [])
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateTenant = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      setIsSubmitting(true)
      await superadminService.createTenant(newTenant)
      setIsModalOpen(false)
      showToast("Inmobiliaria creada correctamente", "success")
      setNewTenant({ business_name: '', cuit: '', dni_responsable: '', first_name: '', last_name: '', admin_email: '', phone: '' })
      await loadTenants()
    } catch (err: any) {
      console.error("Error creating tenant:", err)
      showToast(`No se pudo crear la inmobiliaria: ${err.message || "Error interno del servidor"}`, "error")
    } finally {
      setIsSubmitting(false)
    }
  }

  const filteredTenants = tenants.filter(t => {
    const matchesSearch = t.business_name.toLowerCase().includes(searchTerm.toLowerCase()) || 
                          t.cuit.includes(searchTerm)
    if (filter === 'ALL') return matchesSearch
    return matchesSearch && t.status === filter
  })

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-foreground">Inmobiliarias</h1>
          <p className="text-muted-foreground">Gestión de todos los tenants del sistema.</p>
        </div>
        <Button onClick={() => setIsModalOpen(true)} className="bg-purple-600 hover:bg-purple-700">
          <Plus className="mr-2 h-4 w-4" />
          Nueva Inmobiliaria
        </Button>
      </div>

      <Card>
        <div className="p-4 border-b border-border flex flex-col sm:flex-row gap-4 justify-between">
          <div className="relative max-w-sm w-full">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input 
              type="text" 
              placeholder="Buscar por nombre o CUIT..." 
              className="pl-9" 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
          <div className="flex gap-2">
            <Button variant={filter === 'ALL' ? 'default' : 'outline'} size="sm" onClick={() => setFilter('ALL')}>Todas</Button>
            <Button variant={filter === 'ACTIVE' ? 'default' : 'ghost'} size="sm" className={filter === 'ACTIVE' ? '' : 'text-green-600'} onClick={() => setFilter('ACTIVE')}>Activas</Button>
            <Button variant={filter === 'PENDING' ? 'default' : 'ghost'} size="sm" className={filter === 'PENDING' ? '' : 'text-amber-600'} onClick={() => setFilter('PENDING')}>Pendientes</Button>
            <Button variant={filter === 'SUSPENDED' ? 'default' : 'ghost'} size="sm" className={filter === 'SUSPENDED' ? '' : 'text-red-600'} onClick={() => setFilter('SUSPENDED')}>Suspendidas</Button>
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
                      <Loader2 className="h-6 w-6 animate-spin mx-auto mb-2" />
                      Cargando inmobiliarias...
                    </td>
                  </tr>
                ) : filteredTenants.length === 0 ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">
                      No se encontraron inmobiliarias.
                    </td>
                  </tr>
                ) : (
                  filteredTenants.map(tenant => (
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
                          {tenant.status || 'PENDING'}
                        </Badge>
                      </td>
                      <td className="px-6 py-4 text-muted-foreground">
                        {tenant.created_at ? new Date(tenant.created_at).toLocaleDateString('es-AR') : '-'}
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

      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Nueva Inmobiliaria">
        <form onSubmit={handleCreateTenant} className="space-y-4">
          <div>
            <label className="text-sm font-medium">Razón Social o Nombre Fantasía</label>
            <Input required value={newTenant.business_name} onChange={e => setNewTenant({...newTenant, business_name: e.target.value})} />
          </div>
          <div>
            <label className="text-sm font-medium">CUIT</label>
            <Input required value={newTenant.cuit} onChange={e => setNewTenant({...newTenant, cuit: e.target.value})} />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="text-sm font-medium">Nombre (Resp.)</label>
              <Input required value={newTenant.first_name} onChange={e => setNewTenant({...newTenant, first_name: e.target.value})} />
            </div>
            <div>
              <label className="text-sm font-medium">Apellido (Resp.)</label>
              <Input required value={newTenant.last_name} onChange={e => setNewTenant({...newTenant, last_name: e.target.value})} />
            </div>
          </div>
          <div>
            <label className="text-sm font-medium">DNI (Resp.)</label>
            <Input required value={newTenant.dni_responsable} onChange={e => setNewTenant({...newTenant, dni_responsable: e.target.value})} />
          </div>
          <div>
            <label className="text-sm font-medium">Correo Electrónico (Resp.)</label>
            <Input required type="email" value={newTenant.admin_email} onChange={e => setNewTenant({...newTenant, admin_email: e.target.value})} />
          </div>
          <div>
            <label className="text-sm font-medium">Teléfono (Opcional)</label>
            <Input value={newTenant.phone} onChange={e => setNewTenant({...newTenant, phone: e.target.value})} />
          </div>
          
          <div className="flex justify-end gap-2 pt-4">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>Cancelar</Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
              Crear Inmobiliaria
            </Button>
          </div>
        </form>
      </Modal>

      {toastMessage && (
        <div className={`fixed bottom-4 right-4 text-white px-6 py-3 rounded shadow-lg z-50 transition-opacity ${toastMessage.type === 'error' ? 'bg-red-600' : 'bg-green-600'}`}>
          {toastMessage.msg}
        </div>
      )}
    </div>
  )
}
