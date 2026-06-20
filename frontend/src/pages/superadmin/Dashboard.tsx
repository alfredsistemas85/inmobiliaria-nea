import { useEffect, useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Building2, Activity, ShieldAlert, LifeBuoy, Loader2, Play } from 'lucide-react'
import { superadminService } from '@/services/superadmin'

export default function SuperAdminDashboard() {
  const [stats, setStats] = useState<any>(null)
  const [loading, setLoading] = useState(true)

  const [triggerLoading, setTriggerLoading] = useState(false)
  const [toastMessage, setToastMessage] = useState<string | null>(null)

  useEffect(() => {
    loadStats()
  }, [])

  const showToast = (msg: string) => {
    setToastMessage(msg)
    setTimeout(() => setToastMessage(null), 5000)
  }

  const handleTriggerAdjustments = async () => {
    try {
      setTriggerLoading(true)
      const res = await superadminService.triggerAdjustments()
      const data = await res.json()
      
      if (res.ok) {
        showToast(`Motor ejecutado correctamente.\n${data.contracts_checked} contratos analizados.\n${data.adjustments_generated} ajustes generados.\nTiempo: ${data.execution_time_ms}ms`)
      } else {
        showToast("Error al ejecutar el motor.")
      }
    } catch (err) {
      console.error(err)
      showToast("Error de conexión al ejecutar motor.")
    } finally {
      setTriggerLoading(false)
    }
  }

  const loadStats = async () => {
    try {
      setLoading(true)
      const data = await superadminService.getStats()
      setStats(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return <div className="flex justify-center p-12"><Loader2 className="h-8 w-8 animate-spin text-purple-600" /></div>
  }

  // Fallback to zeros if stats fails to load
  const data = stats || {
    active_tenants: 0,
    suspended_tenants: 0,
    total_tenants: 0,
    mrr_estimado: 0
  }

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
            <div className="text-2xl font-bold">{data.active_tenants}</div>
            <p className="text-xs text-muted-foreground">De un total de {data.total_tenants}</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Inmobiliarias Suspendidas</CardTitle>
            <Building2 className="h-4 w-4 text-amber-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-amber-500">{data.suspended_tenants}</div>
            <p className="text-xs text-muted-foreground">Bloqueadas</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Errores Globales</CardTitle>
            <Activity className="h-4 w-4 text-red-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-500">?</div>
            <p className="text-xs text-muted-foreground">Ver pestaña Monitoreo</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">MRR Estimado</CardTitle>
            <LifeBuoy className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-500">${data.mrr_estimado?.toLocaleString('es-AR')}</div>
            <p className="text-xs text-muted-foreground">Facturación teórica</p>
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

      <Card>
        <CardHeader>
          <CardTitle>Acciones del Sistema</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between p-4 border rounded-md bg-muted/30">
            <div>
              <p className="font-medium text-foreground">Motor de Ajustes</p>
              <p className="text-sm text-muted-foreground">Fuerza la ejecución del scheduler de ajustes para la fecha actual (solo evalúa contratos habilitados para la fecha).</p>
            </div>
            <Button onClick={handleTriggerAdjustments} disabled={triggerLoading} className="gap-2">
              {triggerLoading ? <Loader2 className="h-4 w-4 animate-spin" /> : <Play className="h-4 w-4" />}
              Ejecutar Motor de Ajustes
            </Button>
          </div>
        </CardContent>
      </Card>
      
      {toastMessage && (
        <div className="fixed bottom-4 right-4 bg-slate-800 text-white px-4 py-3 rounded-md shadow-lg z-50 transition-opacity whitespace-pre-line max-w-sm">
          {toastMessage}
        </div>
      )}
    </div>
  )
}
