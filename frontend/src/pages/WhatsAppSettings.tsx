import { useState, useEffect } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { whatsappService } from '@/services/whatsapp'
import { Loader2, Smartphone, QrCode, PowerOff, RefreshCw } from 'lucide-react'

export default function WhatsAppSettings() {
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [instance, setInstance] = useState<any>(null)
  const [qrCode, setQrCode] = useState<string | null>(null)
  const [instanceName, setInstanceName] = useState('')

  const loadStatus = async () => {
    try {
      const res = await whatsappService.getInstanceStatus()
      setInstance(res)
      if (res?.status === 'CONNECTING' && res?.qr_code) {
        setQrCode(res.qr_code)
      } else if (res?.status === 'OPEN') {
        setQrCode(null)
      }
    } catch (err: any) {
      console.error(err)
      setError('Error al obtener el estado de WhatsApp')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    let isMounted = true
    let timeoutId: NodeJS.Timeout

    const pollStatus = async () => {
      await loadStatus()
      if (isMounted) {
        timeoutId = setTimeout(pollStatus, 5000)
      }
    }

    pollStatus()

    return () => {
      isMounted = false
      if (timeoutId) clearTimeout(timeoutId)
    }
  }, [])

  const handleCreate = async () => {
    if (!instanceName.trim()) {
      setError('Debes ingresar un nombre para la instancia')
      return
    }
    setLoading(true)
    setError('')
    try {
      await whatsappService.createInstance(instanceName)
      await loadStatus()
    } catch (err: any) {
      setError('Error al crear instancia')
      setLoading(false)
    }
  }

  const handleGetQr = async () => {
    setLoading(true)
    setError('')
    try {
      await whatsappService.getQr()
      await loadStatus()
    } catch (err: any) {
      setError('Error al obtener QR')
      setLoading(false)
    }
  }

  const handleLogout = async () => {
    setLoading(true)
    setError('')
    try {
      await whatsappService.logoutInstance()
      await loadStatus()
    } catch (err: any) {
      setError('Error al desconectar WhatsApp')
      setLoading(false)
    }
  }

  if (loading && !instance) {
    return <div className="flex justify-center p-8"><Loader2 className="h-8 w-8 animate-spin text-muted-foreground" /></div>
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Smartphone className="h-5 w-5 text-green-500" />
          Integración WhatsApp
        </CardTitle>
        <CardDescription>
          Conecta el WhatsApp de tu agencia para enviar notificaciones y responder mensajes.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {error && (
          <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md">
            {error}
          </div>
        )}

        {!instance ? (
          <div className="space-y-4 bg-muted/50 p-6 rounded-lg border border-border text-center">
            <h3 className="font-medium text-foreground">No hay instancia creada</h3>
            <p className="text-sm text-muted-foreground mb-4">Crea una instancia para comenzar la vinculación.</p>
            <div className="flex flex-col sm:flex-row items-center justify-center gap-2 max-w-sm mx-auto">
              <Input 
                placeholder="Nombre de instancia (ej. agencia-norte)" 
                value={instanceName}
                onChange={e => setInstanceName(e.target.value)}
              />
              <Button onClick={handleCreate} disabled={loading} className="whitespace-nowrap">
                Crear Instancia
              </Button>
            </div>
          </div>
        ) : (
          <div className="space-y-6">
            <div className="flex justify-between items-center bg-muted/50 p-4 rounded-lg border border-border">
              <div>
                <p className="text-sm text-muted-foreground">Instancia</p>
                <p className="font-semibold">{instance.instance_name}</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-muted-foreground">Estado</p>
                <div className="flex items-center gap-2">
                  <span className={`inline-block h-2.5 w-2.5 rounded-full ${instance.status === 'OPEN' ? 'bg-green-500' : instance.status === 'CONNECTING' ? 'bg-amber-500' : 'bg-red-500'}`}></span>
                  <span className="font-semibold">{instance.status || 'Desconocido'}</span>
                </div>
              </div>
            </div>

            {instance.status === 'OPEN' ? (
              <div className="flex flex-col items-center justify-center py-8 text-center space-y-4">
                <div className="h-16 w-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
                  <Smartphone className="h-8 w-8" />
                </div>
                <div>
                  <h3 className="text-xl font-bold text-green-600">WhatsApp Conectado</h3>
                  <p className="text-muted-foreground">Tu número {instance.phone_connected || ''} está vinculado correctamente.</p>
                </div>
                <Button variant="destructive" onClick={handleLogout} disabled={loading} className="mt-4">
                  <PowerOff className="h-4 w-4 mr-2" />
                  Desconectar
                </Button>
              </div>
            ) : (
              <div className="flex flex-col items-center justify-center py-6 text-center space-y-4">
                {qrCode ? (
                  <>
                    <p className="text-muted-foreground text-sm">Escanea este código QR con tu WhatsApp</p>
                    <div className="bg-white p-4 rounded-xl border border-border inline-block shadow-sm">
                      <img src={`data:image/png;base64,${qrCode}`} alt="QR Code" className="w-64 h-64 object-contain" />
                    </div>
                    <Button variant="outline" onClick={handleGetQr} disabled={loading}>
                      <RefreshCw className={`h-4 w-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
                      Actualizar QR
                    </Button>
                  </>
                ) : (
                  <>
                    <p className="text-muted-foreground">La instancia está desconectada. Genera un QR para vincular tu WhatsApp.</p>
                    <Button onClick={handleGetQr} disabled={loading}>
                      <QrCode className="h-4 w-4 mr-2" />
                      Generar QR
                    </Button>
                  </>
                )}
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
