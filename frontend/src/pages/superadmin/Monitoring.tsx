import { useEffect, useState } from 'react'
import { AlertTriangle, Server, Activity } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

export default function SuperAdminMonitoring() {
  const [errors, setErrors] = useState<any[]>([])

  useEffect(() => {
    // Mock errors
    setErrors([
      { id: 1, type: '500_INTERNAL_ERROR', endpoint: '/api/properties', tenant: 'Inmobiliaria Central', time: 'hace 10 minutos', resolved: false },
      { id: 2, type: 'WEBHOOK_FAILURE', endpoint: '/api/whatsapp/webhook', tenant: 'Sur Bienes Raíces', time: 'hace 2 horas', resolved: false },
      { id: 3, type: 'DB_INCONSISTENCY', endpoint: '-', tenant: 'Global', time: 'hace 1 día', resolved: true },
    ])
  }, [])

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight text-foreground">Monitoreo del Sistema</h1>
        <p className="text-muted-foreground">Estado de salud de los componentes y logs de errores globales.</p>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <Card className="border-green-500/50 shadow-sm shadow-green-500/10">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">API Principal</CardTitle>
            <Server className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-500">Operativo</div>
            <p className="text-xs text-muted-foreground">Latencia media: 45ms</p>
          </CardContent>
        </Card>
        <Card className="border-green-500/50 shadow-sm shadow-green-500/10">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Base de Datos</CardTitle>
            <Activity className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-500">Operativo</div>
            <p className="text-xs text-muted-foreground">Conexiones activas: 12</p>
          </CardContent>
        </Card>
        <Card className="border-amber-500/50 shadow-sm shadow-amber-500/10">
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Integración WhatsApp</CardTitle>
            <AlertTriangle className="h-4 w-4 text-amber-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-amber-500">Degradado</div>
            <p className="text-xs text-muted-foreground">1 tenant con webhook fallando</p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Últimos Errores (system_errors)</CardTitle>
        </CardHeader>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  <th className="px-6 py-3 font-medium">Tipo</th>
                  <th className="px-6 py-3 font-medium">Tenant</th>
                  <th className="px-6 py-3 font-medium">Endpoint</th>
                  <th className="px-6 py-3 font-medium">Tiempo</th>
                  <th className="px-6 py-3 font-medium text-right">Estado</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {errors.map(err => (
                  <tr key={err.id} className="hover:bg-muted/50 transition-colors">
                    <td className="px-6 py-4 font-medium text-foreground">{err.type}</td>
                    <td className="px-6 py-4 text-muted-foreground">{err.tenant}</td>
                    <td className="px-6 py-4 text-muted-foreground font-mono text-xs">{err.endpoint}</td>
                    <td className="px-6 py-4 text-muted-foreground">{err.time}</td>
                    <td className="px-6 py-4 text-right">
                      {err.resolved ? (
                        <Badge variant="outline" className="border-green-500 text-green-500">Resuelto</Badge>
                      ) : (
                        <Badge variant="outline" className="border-red-500 text-red-500">Pendiente</Badge>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
