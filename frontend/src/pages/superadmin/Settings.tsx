import { useState, useEffect } from 'react'
import { User, CreditCard, Loader2 } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'
import { fetchApi } from '@/services/api'

export default function SuperAdminSettings() {
  const [activeTab, setActiveTab] = useState('perfil')
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')

  // Perfil
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [email, setEmail] = useState('')

  // Contraseña
  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')

  // SaaS Settings
  const [saasPrice, setSaasPrice] = useState('')

  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    setLoading(true)
    setError('')
    try {
      const userData = await authService.me()
      setFirstName(userData.first_name || '')
      setLastName(userData.last_name || '')
      setEmail(userData.email || '')

      const settingsData = await fetchApi('/system/settings')
      if (settingsData && settingsData.saas_subscription_price) {
        setSaasPrice(settingsData.saas_subscription_price)
      }
    } catch (err: any) {
      setError('Error al cargar la configuración global')
    } finally {
      setLoading(false)
    }
  }

  const handleProfileSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    // Lógica para actualizar perfil (asumiendo que usesService tiene el endpoint o similar, 
    // pero omitido para simplificar dado que el usuario pidió Facturación)
    setSuccess('Funcionalidad de perfil en desarrollo')
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

  const handleSaasPriceSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    setSuccess('')
    try {
      await fetchApi('/system/settings', {
        method: 'PUT',
        body: JSON.stringify({
          saas_subscription_price: saasPrice
        })
      })
      setSuccess('Precio de suscripción actualizado. Se aplicará en los nuevos cobros.')
    } catch (err: any) {
      setError(err.message || 'Error al actualizar el precio')
    } finally {
      setSaving(false)
    }
  }

  if (loading) {
    return <div className="flex justify-center py-12"><Loader2 className="h-8 w-8 animate-spin text-muted-foreground" /></div>
  }

  return (
    <div className="space-y-6 max-w-4xl mx-auto p-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight text-foreground">Ajustes Globales</h1>
        <p className="text-muted-foreground">Administra tu perfil de SuperAdmin y configuraciones base del SaaS.</p>
      </div>

      <div className="flex flex-col md:flex-row gap-6">
        <aside className="w-full md:w-64 shrink-0">
          <nav className="flex md:flex-col gap-1 overflow-x-auto md:overflow-visible pb-2 md:pb-0">
            <button 
              onClick={() => setActiveTab('perfil')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'perfil' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <User className="h-4 w-4" /> Mi Perfil
            </button>
            <button 
              onClick={() => setActiveTab('saas')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'saas' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <CreditCard className="h-4 w-4" /> Facturación SaaS
            </button>
          </nav>
        </aside>

        <div className="flex-1 space-y-6">
          {error && (
            <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md">
              {error}
            </div>
          )}
          {success && (
            <div className="p-4 text-green-700 bg-green-50 border border-green-100 rounded-md">
              {success}
            </div>
          )}

          {activeTab === 'perfil' && (
            <div className="space-y-6">
              <Card>
                <CardHeader>
                  <CardTitle>Perfil de SuperAdmin</CardTitle>
                  <CardDescription>
                    Información personal.
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <form onSubmit={handleProfileSubmit} className="space-y-4">
                    <div className="grid gap-4 sm:grid-cols-2">
                      <div className="space-y-2">
                        <label className="text-sm font-medium">Nombre</label>
                        <Input value={firstName} onChange={e => setFirstName(e.target.value)} disabled />
                      </div>
                      <div className="space-y-2">
                        <label className="text-sm font-medium">Apellido</label>
                        <Input value={lastName} onChange={e => setLastName(e.target.value)} disabled />
                      </div>
                      <div className="space-y-2 sm:col-span-2">
                        <label className="text-sm font-medium">Correo Electrónico</label>
                        <Input type="email" value={email} onChange={e => setEmail(e.target.value)} disabled />
                      </div>
                    </div>
                  </form>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Cambiar Contraseña</CardTitle>
                </CardHeader>
                <CardContent>
                  <form onSubmit={handlePasswordSubmit} className="space-y-4">
                    <div className="space-y-2">
                      <label className="text-sm font-medium">Contraseña Actual</label>
                      <Input type="password" required value={currentPassword} onChange={e => setCurrentPassword(e.target.value)} />
                    </div>
                    <div className="space-y-2">
                      <label className="text-sm font-medium">Nueva Contraseña</label>
                      <Input type="password" required value={newPassword} onChange={e => setNewPassword(e.target.value)} />
                    </div>
                    <Button type="submit" disabled={saving}>
                      {saving ? 'Guardando...' : 'Actualizar Contraseña'}
                    </Button>
                  </form>
                </CardContent>
              </Card>
            </div>
          )}

          {activeTab === 'saas' && (
            <Card>
              <CardHeader>
                <CardTitle>Planes y Facturación</CardTitle>
                <CardDescription>
                  Ajusta el precio que cobras mensualmente a las inmobiliarias por usar la plataforma.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleSaasPriceSubmit} className="space-y-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium text-foreground">Valor Mensual de la Suscripción (ARS)</label>
                    <div className="relative">
                      <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 font-medium">$</span>
                      <Input 
                        type="number" 
                        required 
                        className="pl-8"
                        value={saasPrice} 
                        onChange={e => setSaasPrice(e.target.value)} 
                        placeholder="50000"
                      />
                    </div>
                    <p className="text-xs text-muted-foreground mt-1">
                      El monto que se le cobrará a los clientes a través del botón de "Pagar Mensualidad" en sus paneles.
                    </p>
                  </div>
                  
                  <Button type="submit" className="bg-purple-600 hover:bg-purple-700" disabled={saving}>
                    {saving ? 'Guardando...' : 'Guardar Precio'}
                  </Button>
                </form>
              </CardContent>
            </Card>
          )}

        </div>
      </div>
    </div>
  )
}
