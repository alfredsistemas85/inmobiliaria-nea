import { useParams, Link } from 'react-router-dom'
import { ArrowLeft, Building2, User, Phone, MapPin, Activity, Settings, CheckCircle2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

export default function SuperAdminTenantDetail() {
  const { id } = useParams()
  
  // Mock data
  const tenant = {
    id,
    business_name: 'Inmobiliaria Central',
    cuit: '30-71234567-8',
    status: 'ACTIVE',
    first_name: 'Juan',
    last_name: 'Pérez',
    dni_responsable: '20123456',
    address: 'Av. Corrientes 1234',
    city: 'Resistencia',
    province: 'Chaco',
    phone: '3624123456',
    created_at: '2024-01-10T10:00:00Z',
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
          <Badge variant="outline" className="border-green-500 text-green-500 ml-2">
            {tenant.status}
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
                  {tenant.address}, {tenant.city}, {tenant.province}
                </p>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">Teléfono</p>
                <p className="text-foreground flex items-center gap-1">
                  <Phone className="h-4 w-4 text-muted-foreground" />
                  {tenant.phone}
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
              <Button className="w-full justify-start text-left bg-amber-500 hover:bg-amber-600 text-white">
                Suspender Cuenta
              </Button>
              <Button variant="outline" className="w-full justify-start text-left text-red-600 hover:text-red-700 hover:bg-red-50">
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
                <span className="flex items-center gap-1 text-sm text-green-600 font-medium">
                  <CheckCircle2 className="h-4 w-4" /> Conectado
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Webhooks</span>
                <span className="flex items-center gap-1 text-sm text-green-600 font-medium">
                  <CheckCircle2 className="h-4 w-4" /> Saludables
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Errores (24h)</span>
                <span className="flex items-center gap-1 text-sm text-muted-foreground font-medium">
                  0 registrados
                </span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
