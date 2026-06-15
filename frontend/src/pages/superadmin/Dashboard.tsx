import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Building2, Activity, ShieldAlert, LifeBuoy } from 'lucide-react'

export default function SuperAdminDashboard() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight text-foreground">SuperAdmin Dashboard</h1>
        <p className="text-muted-foreground">Visión global del sistema SaaS.</p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Inmobiliarias Activas</CardTitle>
            <Building2 className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">12</div>
            <p className="text-xs text-muted-foreground">+2 este mes</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Inmobiliarias Pendientes</CardTitle>
            <Building2 className="h-4 w-4 text-amber-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-amber-500">3</div>
            <p className="text-xs text-muted-foreground">Requieren validación</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Errores Globales (24h)</CardTitle>
            <Activity className="h-4 w-4 text-red-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-500">5</div>
            <p className="text-xs text-muted-foreground">Fallos de Webhook o 500s</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Tickets Abiertos</CardTitle>
            <LifeBuoy className="h-4 w-4 text-blue-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-500">8</div>
            <p className="text-xs text-muted-foreground">Esperando respuesta</p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Estado del Sistema</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 border rounded-md">
              <div className="flex items-center gap-3">
                <ShieldAlert className="h-5 w-5 text-green-500" />
                <div>
                  <p className="font-medium text-foreground">API Backend</p>
                  <p className="text-sm text-muted-foreground">Operativo</p>
                </div>
              </div>
              <span className="text-sm text-green-500 font-medium">Online</span>
            </div>
            <div className="flex items-center justify-between p-4 border rounded-md">
              <div className="flex items-center gap-3">
                <ShieldAlert className="h-5 w-5 text-green-500" />
                <div>
                  <p className="font-medium text-foreground">Servicio WhatsApp (Evolution)</p>
                  <p className="text-sm text-muted-foreground">Operativo</p>
                </div>
              </div>
              <span className="text-sm text-green-500 font-medium">Online</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
