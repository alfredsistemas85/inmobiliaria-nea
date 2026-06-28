import { useState } from 'react'
import { Settings, CreditCard, FileText, Bell, CheckCircle2, XCircle, AlertTriangle } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

export function IntegrationsTab() {
  const [saving, setSaving] = useState(false)
  
  // Fake state for demonstration. In a real app this would load from backend settings.
  const [afipStatus, setAfipStatus] = useState<'connected' | 'error' | 'disconnected'>('connected')
  const [mpStatus, setMpStatus] = useState<'connected' | 'error' | 'disconnected'>('connected')
  const [autoPayEnabled, setAutoPayEnabled] = useState(true)
  const [webhooksEnabled, setWebhooksEnabled] = useState(true)

  const handleSave = () => {
    setSaving(true)
    setTimeout(() => {
      setSaving(false)
      alert('Configuración guardada exitosamente.')
    }, 1000)
  }

  return (
    <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500 max-w-5xl">
      
      <div className="grid md:grid-cols-2 gap-6">
        {/* AFIP Card */}
        <Card>
          <CardHeader>
            <div className="flex justify-between items-start">
              <div className="flex items-center gap-2">
                <div className="p-2 bg-primary/10 rounded-md">
                  <FileText className="h-5 w-5 text-primary" />
                </div>
                <div>
                  <CardTitle className="text-lg">Facturación AFIP</CardTitle>
                  <CardDescription>Conexión a WebServices de ARCA/AFIP</CardDescription>
                </div>
              </div>
              <div className="flex items-center gap-1.5 text-sm font-medium">
                {afipStatus === 'connected' && <span className="text-emerald-600 flex items-center gap-1"><CheckCircle2 className="h-4 w-4"/> Conectado (Dummy)</span>}
                {afipStatus === 'disconnected' && <span className="text-muted-foreground flex items-center gap-1"><XCircle className="h-4 w-4"/> Desconectado</span>}
                {afipStatus === 'error' && <span className="text-red-600 flex items-center gap-1"><AlertTriangle className="h-4 w-4"/> Error de Certificado</span>}
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none">CUIT Inmobiliaria</label>
              <Input defaultValue="30-12345678-9" />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none">Punto de Venta</label>
              <Input defaultValue="0001" type="number" />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none">Certificado (.crt / .pem)</label>
              <Input type="file" />
            </div>
          </CardContent>
          <CardFooter className="border-t pt-4">
            <Button variant="outline" className="w-full">Probar Conexión AFIP</Button>
          </CardFooter>
        </Card>

        {/* Mercado Pago Card */}
        <Card>
          <CardHeader>
            <div className="flex justify-between items-start">
              <div className="flex items-center gap-2">
                <div className="p-2 bg-blue-100 rounded-md dark:bg-blue-900/30">
                  <CreditCard className="h-5 w-5 text-blue-600 dark:text-blue-400" />
                </div>
                <div>
                  <CardTitle className="text-lg">Mercado Pago</CardTitle>
                  <CardDescription>Pasarela de pagos y Webhooks</CardDescription>
                </div>
              </div>
              <div className="flex items-center gap-1.5 text-sm font-medium">
                {mpStatus === 'connected' && <span className="text-emerald-600 flex items-center gap-1"><CheckCircle2 className="h-4 w-4"/> Sandbox Activo</span>}
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none">Access Token (Producción)</label>
              <Input type="password" defaultValue="APP_USR-12345678-..." />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium leading-none">Access Token (Pruebas)</label>
              <Input type="password" defaultValue="TEST-12345678-..." />
            </div>
            <div className="flex items-center justify-between mt-6 p-4 border rounded-lg bg-muted/30">
              <div className="space-y-0.5">
                <label className="text-base font-semibold">Recibir Webhooks</label>
                <p className="text-sm text-muted-foreground">Procesa pagos automáticamente</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" className="sr-only peer" checked={webhooksEnabled} onChange={(e) => setWebhooksEnabled(e.target.checked)} />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>
          </CardContent>
          <CardFooter className="border-t pt-4">
            <Button variant="outline" className="w-full">Generar Link de Prueba</Button>
          </CardFooter>
        </Card>

        {/* Automatización */}
        <Card className="md:col-span-2">
          <CardHeader>
            <div className="flex items-center gap-2">
              <div className="p-2 bg-amber-100 rounded-md dark:bg-amber-900/30">
                <Settings className="h-5 w-5 text-amber-600 dark:text-amber-400" />
              </div>
              <div>
                <CardTitle className="text-lg">Motor de Automatización (Workers)</CardTitle>
                <CardDescription>Configura los procesos en segundo plano del ERP</CardDescription>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="grid md:grid-cols-2 gap-6">
              <div className="flex items-start justify-between p-4 border rounded-lg">
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <CreditCard className="h-4 w-4 text-primary" />
                    <label className="text-base font-semibold">Débito Automático (Autopay)</label>
                  </div>
                  <p className="text-sm text-muted-foreground">Ejecuta cobros programados de inquilinos con tarjetas adheridas en el día del vencimiento.</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input type="checkbox" className="sr-only peer" checked={autoPayEnabled} onChange={(e) => setAutoPayEnabled(e.target.checked)} />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
                </label>
              </div>
              <div className="flex items-start justify-between p-4 border rounded-lg">
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <Bell className="h-4 w-4 text-primary" />
                    <label className="text-base font-semibold">Recordatorios (WhatsApp)</label>
                  </div>
                  <p className="text-sm text-muted-foreground">Envía recordatorios de pago automáticos 3 días antes del vencimiento vía Evolution API.</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input type="checkbox" className="sr-only peer" checked={true} readOnly />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-primary"></div>
                </label>
              </div>
            </div>
          </CardContent>
          <CardFooter className="border-t pt-4 flex justify-end">
            <Button onClick={handleSave} disabled={saving} className="w-full md:w-auto">
              {saving ? 'Guardando...' : 'Guardar Configuración'}
            </Button>
          </CardFooter>
        </Card>
      </div>

    </div>
  )
}
