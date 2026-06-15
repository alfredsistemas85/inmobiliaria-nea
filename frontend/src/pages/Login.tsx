import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Home, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from '@/components/ui/card'
import { authService } from '@/services/auth'

export default function Login() {
  const navigate = useNavigate()
  const [identifier, setIdentifier] = useState('')
  const [password, setPassword] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError('')
    
    try {
      const response = await authService.login({ identifier, password })
      localStorage.setItem('token', response.access_token)
      localStorage.setItem('refresh_token', response.refresh_token)
      if (response.tenant_id) localStorage.setItem('tenant_id', response.tenant_id)
      localStorage.setItem('user', JSON.stringify({
        id: response.user_id,
        role: response.role,
        first_name: response.first_name,
        last_name: response.last_name,
      }))
      navigate('/dashboard')
    } catch (err: any) {
      if (err.status === 403) {
        setError('Debes verificar tu correo para poder ingresar. Por favor revisa tu bandeja de entrada.')
      } else {
        setError(err.message || 'Error al iniciar sesión')
      }
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div className="flex flex-col items-center">
          <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-blue-600 text-white shadow-lg">
            <Home className="h-6 w-6" />
          </div>
          <h2 className="mt-6 text-center text-3xl font-bold tracking-tight text-foreground">
            InmobiCRM
          </h2>
          <p className="mt-2 text-center text-sm text-muted-foreground">
            Sistema de Gestión Inmobiliaria
          </p>
        </div>

        <Card className="mt-8">
          <CardHeader>
            <CardTitle>Iniciar Sesión</CardTitle>
            <CardDescription>
              Ingresa tus credenciales para acceder al sistema.
            </CardDescription>
          </CardHeader>
          <form onSubmit={handleLogin}>
            <CardContent className="space-y-4">
              {error && (
                <div className="p-3 text-sm text-red-600 bg-red-50 border border-red-100 rounded-md flex items-center gap-2">
                  <AlertCircle className="h-4 w-4" />
                  {error}
                </div>
              )}
              <div className="space-y-2">
                <label className="text-sm font-medium text-foreground" htmlFor="identifier">
                  Correo Electrónico o CUIT
                </label>
                <Input 
                  id="identifier" 
                  type="text" 
                  placeholder="ejemplo@inmobicrm.com o 30712345678" 
                  required 
                  value={identifier}
                  onChange={(e) => setIdentifier(e.target.value)}
                />
              </div>
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <label className="text-sm font-medium text-foreground" htmlFor="password">
                    Contraseña
                  </label>
                  <a href="#" className="text-xs text-blue-600 hover:underline">
                    ¿Olvidaste tu contraseña?
                  </a>
                </div>
                <Input 
                  id="password" 
                  type="password" 
                  required 
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                />
              </div>
            </CardContent>
            <CardFooter>
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? 'Iniciando sesión...' : 'Ingresar al Dashboard'}
              </Button>
            </CardFooter>
          </form>
        </Card>
      </div>
    </div>
  )
}
