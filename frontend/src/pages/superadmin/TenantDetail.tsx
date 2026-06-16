import { useEffect, useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { ArrowLeft, Building2, User, Phone, MapPin, Activity, Settings, CheckCircle2, AlertCircle, Loader2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { superadminService } from '@/services/superadmin'
import { cn } from '@/lib/utils'

export default function SuperAdminTenantDetail() {
  const { id } = useParams<{ id: string }>()
  const [tenant, setTenant] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [updating, setUpdating] = useState(false)

  useEffect(() => {
    if (id) loadTenant()
  }, [id])

  const loadTenant = async () => {
    try {
      setLoading(true)
      const data = await superadminService.getTenant(id!)
      setTenant(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleStatusChange = async (newStatus: string) => {
    if (!confirm(`¿Estás seguro de cambiar el estado a ${newStatus}?`)) return;
    try {
      setUpdating(true)
      await superadminService.updateTenantStatus(id!, newStatus)
      await loadTenant()
    } catch (err) {
      console.error(err)
      alert("Error al cambiar el estado")
    } finally {
      setUpdating(false)
    }
  }

  if (loading) {
    return <div className="flex justify-center p-12"><Loader2 className="h-8 w-8 animate-spin text-purple-600" /></div>
  }

  if (!tenant) {
    return <div className="text-center p-12 text-muted-foreground">Inmobiliaria no encontrada.</div>
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link to="/superadmin/tenants">
          <Button variant="ghost" size="sm" className="gap-2">
            <ArrowLeft className="h-4 w-4" />
            Volver
          </Button>
        </Link>
        <h1 className="text-3xl font-bold tracking-tight text-foreground flex items-center gap-3">
          {tenant.business_name}
          <Badge variant="outline" className={cn(
            tenant.status === 'ACTIVE' && "border-green-500 text-green-500",
            tenant.status === 'PENDING' && "border-amber-500 text-amber-500",
            tenant.status === 'SUSPENDED' && "border-red-500 text-red-500",
          )}>
            {tenant.status || 'PENDING'}
          </Badge>
        </h1>
      </div>

      <div className="grid gap-6 md:grid-cols-3">
        <div className="space-y-6 md:col-span-2">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Building2 className="h-5 w-5 text-purple-600" />
                Datos de la Empresa
              </CardTitle>
            </CardHeader>
            <CardContent className="grid gap-4 sm:grid-cols-2">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Razón Social</p>
                <p className="text-foreground font-medium">{tenant.business_name}</p>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">CUIT</p>
                <p className="text-foreground font-medium">{tenant.cuit}</p>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">Dirección</p>
                <p className="text-foreground flex items-center gap-1">
                  <MapPin className="h-4 w-4 text-muted-foreground" />
                  {tenant.address || '-'}, {tenant.city || '-'}, {tenant.province || '-'}
                </p>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">Teléfono</p>
                <p className="text-foreground flex items-center gap-1">
                  <Phone className="h-4 w-4 text-muted-foreground" />
                  {tenant.phone || '-'}
                </p>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <User className="h-5 w-5 text-purple-600" />
                Responsable
              </CardTitle>
            </CardHeader>
            <CardContent className="grid gap-4 sm:grid-cols-2">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Nombre y Apellido</p>
                <p className="text-foreground font-medium">{tenant.first_name} {tenant.last_name}</p>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">DNI</p>
                <p className="text-foreground font-medium">{tenant.dni_responsable}</p>
              </div>
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Settings className="h-5 w-5 text-purple-600" />
                Acciones Administrativas
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              {tenant.status !== 'ACTIVE' && (
                <Button 
                  onClick={() => handleStatusChange('ACTIVE')} 
                  disabled={updating}
                  className="w-full justify-start text-left bg-green-600 hover:bg-green-700 text-white"
                >
                  {updating ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                  Activar Cuenta
                </Button>
              )}
              {tenant.status !== 'SUSPENDED' && (
                <Button 
                  onClick={() => handleStatusChange('SUSPENDED')} 
                  disabled={updating}
                  className="w-full justify-start text-left bg-amber-500 hover:bg-amber-600 text-white"
                >
                  {updating ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                  Suspender Cuenta
                </Button>
              )}
              <Button 
                onClick={() => handleStatusChange('DELETED')} 
                disabled={updating}
                variant="outline" 
                className="w-full justify-start text-left text-red-600 hover:text-red-700 hover:bg-red-50"
              >
                {updating ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                Eliminar Inmobiliaria
              </Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Activity className="h-5 w-5 text-purple-600" />
                Estado Técnico
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">WhatsApp API</span>
                <span className="flex items-center gap-1 text-sm text-muted-foreground">
                  Desconocido (No integrado en vista SA)
                </span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
