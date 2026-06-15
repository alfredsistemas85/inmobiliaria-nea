import { useState, useEffect } from 'react'
import { User, Bell, Shield, Smartphone, Globe, CreditCard, Loader2 } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'
import { usersService } from '@/services/users'
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
            <button className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left">
              <Bell className="h-4 w-4" /> Notificaciones
            </button>
            <button 
              onClick={() => setActiveTab('whatsapp')}
              className={`flex items-center gap-2 px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors w-full text-left ${activeTab === 'whatsapp' ? 'bg-blue-50 text-blue-700' : 'text-muted-foreground hover:bg-background hover:text-foreground'}`}
            >
              <Smartphone className="h-4 w-4" /> Integración WhatsApp
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

          {activeTab === 'whatsapp' && <WhatsAppSettings />}

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
