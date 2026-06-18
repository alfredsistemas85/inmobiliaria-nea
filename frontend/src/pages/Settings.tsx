import { useState, useEffect } from 'react'
import { User, Bell, Shield, Smartphone, Globe, CreditCard, Loader2 } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'
import { usersService } from '@/services/users'
import { fetchApi } from '@/services/api'
import WhatsAppSettings from './WhatsAppSettings'

export default function Settings() {
  const [activeTab, setActiveTab] = useState('perfil')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  const [user, setUser] = useState<any>(null)

  // Profile Form
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [email, setEmail] = useState('')

  // Password Form
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')

  // Payments Form
  const [mpAccessToken, setMpAccessToken] = useState('')
  const [mpPublicKey, setMpPublicKey] = useState('')
  const [cbu, setCbu] = useState('')
  const [alias, setAlias] = useState('')

  useEffect(() => {
    loadUser()
  }, [])

  const loadUser = async () => {
    setLoading(true)
    setError('')
    try {
      const data = await authService.me()
      setUser(data)
      setFirstName(data.first_name || '')
      setLastName(data.last_name || '')
      setEmail(data.email || '')
    } catch (err: any) {
      setError('Error al cargar perfil')
    } finally {
      setLoading(false)
    }
  }

  const handleProfileSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSuccess('')
    try {
      await usersService.updateUser(user.id, {
        first_name: firstName,
        last_name: lastName,
        email: email
      })
      setSuccess('Perfil actualizado correctamente')
    } catch (err: any) {
      setError(err.message || 'Error al actualizar perfil')
    } finally {
      setSaving(false)
    }
  }

  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSuccess('')
    try {
      await authService.changePassword({
        current_password: currentPassword,
        new_password: newPassword
      })
      setSuccess('Contraseña actualizada correctamente')
      setCurrentPassword('')
      setNewPassword('')
    } catch (err: any) {
      setError(err.message || 'Error al cambiar contraseña')
    } finally {
      setSaving(false)
    }
  }

  const handlePaymentsSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSuccess('')
    try {
      await fetchApi('/payments/config', {
        method: 'PUT',
        body: JSON.stringify({
          mp_access_token: mpAccessToken,
          mp_public_key: mpPublicKey,
          cbu,
          alias
        })
      })
      setSuccess('Configuración de pagos guardada correctamente')
    } catch (err: any) {
      setError(err.message || 'Error al guardar configuración de pagos')
    } finally {
      setSaving(false)
    }
  }

  if (loading) {
    return <div className="flex justify-center py-12"><Loader2 className="h-8 w-8 animate-spin text-muted-foreground" /></div>
  }

  return (
    <div className="space-y-6 max-w-4xl mx-auto">
      <div>
        <h1 className="text-2xl font-bold tracking-tight text-foreground">Configuración</h1>
        <p className="text-muted-foreground">Administra las preferencias de tu cuenta y el sistema.</p>
      </div>

      <div className="flex flex-col md:flex-row gap-6">
        <aside className="w-full md:w-64 shrink-0">
          <nav className="flex md:flex-col gap-1 overflow-x-auto md:overflow-visible pb-2 md:pb-0">
            <button 
              onClick={() => setActiveTab('perfil')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'perfil' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <User className="h-4 w-4" /> Perfil
            </button>
            <button 
              onClick={() => setActiveTab('seguridad')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'seguridad' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <Shield className="h-4 w-4" /> Seguridad
            </button>
            <button 
              onClick={() => setActiveTab('notificaciones')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'notificaciones' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <Bell className="h-4 w-4" /> Notificaciones
            </button>
            <button 
              onClick={() => setActiveTab('whatsapp')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'whatsapp' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <Smartphone className="h-4 w-4" /> Integración WhatsApp
            </button>
            <button 
              onClick={() => setActiveTab('pagos')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'pagos' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <CreditCard className="h-4 w-4" /> Configuración de Pagos
            </button>
            <button 
              onClick={() => setActiveTab('suscripcion')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'suscripcion' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <CreditCard className="h-4 w-4" /> Suscripción (SaaS)
            </button>
            <button 
              onClick={() => setActiveTab('integrations')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'integrations' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <Globe className="h-4 w-4" /> Integraciones
            </button>
          </nav>
        </aside>

        <div className="flex-1 space-y-6">
          {error && activeTab !== 'whatsapp' && (
            <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md">
              {error}
            </div>
          )}
          {success && activeTab !== 'whatsapp' && (
            <div className="p-4 text-green-700 bg-green-50 border border-green-100 rounded-md">
              {success}
            </div>
          )}

          {activeTab === 'notificaciones' && (
            <Card>
              <CardHeader>
                <CardTitle>Preferencias de Notificaciones</CardTitle>
                <CardDescription>
                  Controla qué notificaciones recibirás dentro de la plataforma.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                {[
                  { label: 'Nuevo Lead asignado', description: 'Recibe una notificación cuando te asignen un lead.' },
                  { label: 'Nueva cita agendada', description: 'Recibe una notificación cuando se agende una visita.' },
                  { label: 'Mensaje de WhatsApp', description: 'Recibe alertas de nuevos mensajes entrantes.' },
                  { label: 'Vencimiento de contrato', description: 'Alerta 30 días antes del vencimiento de un contrato.' },
                ].map((item, i) => (
                  <div key={i} className="flex items-center justify-between py-3 border-b border-border last:border-0">
                    <div>
                      <p className="text-sm font-medium text-foreground">{item.label}</p>
                      <p className="text-xs text-muted-foreground">{item.description}</p>
                    </div>
                    <div className="text-xs text-muted-foreground italic">Activado</div>
                  </div>
                ))}
                <p className="text-xs text-muted-foreground pt-2">La configuración granular de notificaciones estará disponible próximamente.</p>
              </CardContent>
            </Card>
          )}

          {activeTab === 'whatsapp' && <WhatsAppSettings />}

          {activeTab === 'integrations' && (
            <Card>
              <CardHeader>
                <CardTitle>Integraciones</CardTitle>
                <CardDescription>Conecta servicios externos para potenciar tu CRM.</CardDescription>
              </CardHeader>
              <CardContent className="space-y-6">
                <div className="flex items-center justify-between border p-4 rounded-lg">
                  <div className="flex items-center gap-4">
                    <div className="bg-white p-2 rounded-full border shadow-sm">
                      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path fillRule="evenodd" clipRule="evenodd" d="M23.52 12.2727C23.52 11.4218 23.4436 10.6036 23.3018 9.81818H12V14.4545H18.4582C18.18 15.9491 17.3291 17.2036 16.0636 18.0545V21.0545H19.9473C22.2164 18.9545 23.52 15.8945 23.52 12.2727Z" fill="#4285F4"/>
                        <path fillRule="evenodd" clipRule="evenodd" d="M12 24C15.24 24 17.9673 22.9255 19.9473 21.0545L16.0636 18.0545C14.9945 18.7745 13.6145 19.2 12 19.2C8.87455 19.2 6.22909 17.0891 5.27455 14.2582H1.27636V17.3564C3.24545 21.2673 7.30909 24 12 24Z" fill="#34A853"/>
                        <path fillRule="evenodd" clipRule="evenodd" d="M5.27455 14.2582C5.02909 13.5218 4.89273 12.7691 4.89273 12C4.89273 11.2309 5.02909 10.4782 5.27455 9.74182V6.64364H1.27636C0.463636 8.26364 0 10.08 0 12C0 13.92 0.463636 15.7364 1.27636 17.3564L5.27455 14.2582Z" fill="#FBBC05"/>
                        <path fillRule="evenodd" clipRule="evenodd" d="M12 4.8C13.7673 4.8 15.3491 5.40545 16.5982 6.59455L20.0291 3.16364C17.9618 1.23818 15.24 0 12 0C7.30909 0 3.24545 2.73273 1.27636 6.64364L5.27455 9.74182C6.22909 6.91091 8.87455 4.8 12 4.8Z" fill="#EA4335"/>
                      </svg>
                    </div>
                    <div>
                      <h4 className="font-semibold text-gray-900">Google Calendar</h4>
                      <p className="text-sm text-gray-500">Sincroniza tus visitas y reuniones automáticamente</p>
                    </div>
                  </div>
                  <Button variant="outline" onClick={() => window.location.href = `${import.meta.env.VITE_API_URL}/api/calendar/google/connect`}>
                    Conectar
                  </Button>
                </div>
              </CardContent>
            </Card>
          )}

          {activeTab === 'pagos' && (
            <Card>
              <CardHeader>
                <CardTitle>Configuración de Pagos</CardTitle>
                <CardDescription>
                  Configura tus credenciales de Mercado Pago y datos bancarios para cobrar a tus clientes.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <form onSubmit={handlePaymentsSubmit} className="space-y-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Mercado Pago Access Token</label>
                    <Input placeholder="APP_USR-..." type="password" value={mpAccessToken} onChange={e => setMpAccessToken(e.target.value)} />
                  </div>
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Mercado Pago Public Key</label>
                    <Input placeholder="APP_USR-..." value={mpPublicKey} onChange={e => setMpPublicKey(e.target.value)} />
                  </div>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <label className="text-sm font-medium">CBU / CVU Bancario</label>
                      <Input placeholder="0000000000000000000000" value={cbu} onChange={e => setCbu(e.target.value)} />
                    </div>
                    <div className="space-y-2">
                      <label className="text-sm font-medium">Alias</label>
                      <Input placeholder="mi.alias.banco" value={alias} onChange={e => setAlias(e.target.value)} />
                    </div>
                  </div>
                  <Button type="submit" className="mt-4" disabled={saving}>
                    {saving ? 'Guardando...' : 'Guardar Credenciales'}
                  </Button>
                </form>
              </CardContent>
            </Card>
          )}

          {activeTab === 'suscripcion' && (
            <Card>
              <CardHeader>
                <CardTitle>Suscripción y Facturación</CardTitle>
                <CardDescription>
                  Administra el pago de tu mensualidad por el uso de la plataforma InmoNeaCRM.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="bg-purple-50 p-6 rounded-lg border border-purple-100 flex flex-col md:flex-row items-center justify-between">
                  <div>
                    <h3 className="text-lg font-bold text-purple-900">Plan Profesional</h3>
                    <p className="text-purple-700">Disfruta de todas las características sin límites.</p>
                  </div>
                  <div className="mt-4 md:mt-0 text-center md:text-right">
                    <span className="text-2xl font-bold text-purple-900">$50.000</span>
                    <span className="text-sm text-purple-700"> / mes</span>
                  </div>
                </div>
                
                <Button 
                  className="w-full sm:w-auto bg-purple-600 hover:bg-purple-700"
                  onClick={async () => {
                    try {
                      setSaving(true);
                      const res = await fetchApi('/payments/checkout/subscription', { method: 'POST' });
                      if (res && res.init_point) {
                        window.location.href = res.init_point;
                      } else {
                        throw new Error('No se pudo generar el link de pago');
                      }
                    } catch (err: any) {
                      setError(err.message || 'Error al conectar con Mercado Pago');
                      setSaving(false);
                    }
                  }}
                  disabled={saving}
                >
                  {saving ? <Loader2 className="mr-2 h-4 w-4 animate-spin" /> : null}
                  {saving ? 'Redirigiendo a Mercado Pago...' : 'Pagar Mensualidad'}
                </Button>
                <p className="text-xs text-muted-foreground mt-4 text-center sm:text-left">
                  Al hacer clic serás redirigido a Mercado Pago para completar tu suscripción de forma segura. No cobramos comisiones adicionales.
                </p>
              </CardContent>
            </Card>
          )}

          {activeTab === 'perfil' && (
            <Card>
              <CardHeader>
                <CardTitle>Perfil de Usuario</CardTitle>
                <CardDescription>
                  Actualiza tu información personal y de contacto público.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleProfileSubmit} className="space-y-4">
                  <div className="flex items-center gap-4 mb-6">
                    <div className="h-20 w-20 rounded-full bg-blue-100 flex items-center justify-center text-blue-700 text-2xl font-bold uppercase">
                      {firstName?.[0]}{lastName?.[0]}
                    </div>
                  </div>
                  
                  <div className="grid gap-4 sm:grid-cols-2">
                    <div className="space-y-2">
                      <label className="text-sm font-medium text-foreground">Nombre</label>
                      <Input required value={firstName} onChange={e => setFirstName(e.target.value)} />
                    </div>
                    <div className="space-y-2">
                      <label className="text-sm font-medium text-foreground">Apellido</label>
                      <Input required value={lastName} onChange={e => setLastName(e.target.value)} />
                    </div>
                    <div className="space-y-2 sm:col-span-2">
                      <label className="text-sm font-medium text-foreground">Correo Electrónico</label>
                      <Input required type="email" value={email} onChange={e => setEmail(e.target.value)} />
                    </div>
                  </div>

                  <div className="pt-4 flex justify-end">
                    <Button type="submit" disabled={saving}>
                      {saving ? 'Guardando...' : 'Guardar Cambios'}
                    </Button>
                  </div>
                </form>
              </CardContent>
            </Card>
          )}

          {activeTab === 'seguridad' && (
            <Card>
              <CardHeader>
                <CardTitle>Seguridad</CardTitle>
                <CardDescription>
                  Cambia tu contraseña.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form onSubmit={handlePasswordSubmit} className="space-y-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium text-foreground">Contraseña Actual</label>
                    <Input required type="password" value={currentPassword} onChange={e => setCurrentPassword(e.target.value)} />
                  </div>
                  <div className="space-y-2">
                    <label className="text-sm font-medium text-foreground">Nueva Contraseña</label>
                    <Input required type="password" value={newPassword} onChange={e => setNewPassword(e.target.value)} />
                  </div>
                  <div className="pt-4 flex justify-end">
                    <Button type="submit" disabled={saving}>
                      {saving ? 'Cambiando...' : 'Cambiar Contraseña'}
                    </Button>
                  </div>
                </form>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </div>
  )
}
