import { User, Bell, Shield, Smartphone, Globe, CreditCard } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

export default function Settings() {
  return (
    <div className="space-y-6 max-w-4xl">
      <div>
        <h1 className="text-2xl font-bold tracking-tight text-foreground">Configuración</h1>
        <p className="text-muted-foreground">Administra las preferencias de tu cuenta y el sistema.</p>
      </div>

      <div className="flex flex-col md:flex-row gap-6">
        <aside className="w-full md:w-64 shrink-0">
          <nav className="flex md:flex-col gap-1 overflow-x-auto md:overflow-visible pb-2 md:pb-0">
            <a href="#" className="flex items-center gap-2 bg-blue-50 text-blue-700 px-3 py-2.5 rounded-md text-sm font-medium shrink-0">
              <User className="h-4 w-4" /> Perfil
            </a>
            <a href="#" className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors">
              <Bell className="h-4 w-4" /> Notificaciones
            </a>
            <a href="#" className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors">
              <Shield className="h-4 w-4" /> Seguridad
            </a>
            <a href="#" className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors">
              <Smartphone className="h-4 w-4" /> Integración WhatsApp
            </a>
            <a href="#" className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors">
              <Globe className="h-4 w-4" /> Sitio Web
            </a>
            <a href="#" className="flex items-center gap-2 text-muted-foreground hover:bg-background hover:text-foreground px-3 py-2.5 rounded-md text-sm font-medium shrink-0 transition-colors">
              <CreditCard className="h-4 w-4" /> Facturación
            </a>
          </nav>
        </aside>

        <div className="flex-1 space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Perfil de Usuario</CardTitle>
              <CardDescription>
                Actualiza tu información personal y de contacto público.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center gap-4 mb-4">
                <div className="h-20 w-20 rounded-full bg-blue-100 flex items-center justify-center text-blue-700 text-2xl font-bold">
                  JD
                </div>
                <Button variant="outline">Cambiar Avatar</Button>
              </div>
              
              <div className="grid gap-4 sm:grid-cols-2">
                <div className="space-y-2">
                  <label className="text-sm font-medium text-foreground">Nombre</label>
                  <Input defaultValue="John" />
                </div>
                <div className="space-y-2">
                  <label className="text-sm font-medium text-foreground">Apellido</label>
                  <Input defaultValue="Doe" />
                </div>
                <div className="space-y-2 sm:col-span-2">
                  <label className="text-sm font-medium text-foreground">Correo Electrónico</label>
                  <Input defaultValue="john.doe@inmobicrm.com" type="email" />
                </div>
                <div className="space-y-2 sm:col-span-2">
                  <label className="text-sm font-medium text-foreground">Teléfono (WhatsApp)</label>
                  <Input defaultValue="+54 9 11 1234-5678" />
                </div>
              </div>

              <div className="pt-4 flex justify-end">
                <Button>Guardar Cambios</Button>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Preferencias de la Agencia</CardTitle>
              <CardDescription>
                Ajustes generales para tu agencia inmobiliaria.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium text-foreground">Nombre de la Agencia</label>
                <Input defaultValue="Inmobiliaria Norte" />
              </div>
              <div className="pt-4 flex justify-end">
                <Button>Guardar Cambios</Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
