import { useState, useEffect } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { Loader2, KeyRound } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'

export default function Onboarding() {
  const [searchParams] = useSearchParams()
  const token = searchParams.get('token')
  const navigate = useNavigate()
  
  const [password, setPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState(false)

  useEffect(() => {
    if (!token) {
      setError('Enlace inválido o expirado. Falta el token de configuración.')
    }
  }, [token])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (password !== confirmPassword) {
      setError('Las contraseñas no coinciden')
      return
    }
    
    if (password.length < 6) {
      setError('La contraseña debe tener al menos 6 caracteres')
      return
    }

    try {
      setLoading(true)
      setError('')
      const res = await authService.setupPassword(token as string, password)
      
      // En este proyecto fetchApi a veces retorna la response directamente o null.
      // Aquí asumimos que si no lanza excepcion o devuelve json ok, está bien.
      if (res && res.error) {
         setError(res.error)
         return
      }
      
      setSuccess(true)
      setTimeout(() => {
        navigate('/login')
      }, 3000)
    } catch (err: any) {
      setError(err.message || 'Error de conexión con el servidor')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-muted/40 px-4 py-12">
      <div className="w-full max-w-md space-y-8">
        <Card className="shadow-lg border-t-4 border-t-purple-600">
          <CardHeader className="text-center space-y-2">
            <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-purple-100 mb-4">
              <KeyRound className="h-6 w-6 text-purple-600" />
            </div>
            <CardTitle className="text-2xl font-bold tracking-tight">Bienvenido a la Plataforma</CardTitle>
            <CardDescription>
              {success 
                ? 'Contraseña configurada con éxito' 
                : 'Configura tu nueva contraseña para acceder a tu panel de inmobiliaria'}
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            {success ? (
              <div className="rounded-md bg-green-50 p-4 text-center">
                <p className="text-sm font-medium text-green-800">
                  ¡Todo listo! Redirigiendo al panel de acceso...
                </p>
              </div>
            ) : (
              <form onSubmit={handleSubmit} className="space-y-4">
                {error && (
                  <div className="rounded-md bg-red-50 p-3">
                    <p className="text-sm font-medium text-red-800">{error}</p>
                  </div>
                )}
                
                <div className="space-y-2">
                  <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" htmlFor="password">
                    Nueva Contraseña
                  </label>
                  <Input
                    id="password"
                    type="password"
                    placeholder="••••••••"
                    required
                    disabled={!token || loading}
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                  />
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" htmlFor="confirmPassword">
                    Confirmar Contraseña
                  </label>
                  <Input
                    id="confirmPassword"
                    type="password"
                    placeholder="••••••••"
                    required
                    disabled={!token || loading}
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                  />
                </div>

                <Button 
                  type="submit" 
                  className="w-full bg-purple-600 hover:bg-purple-700 mt-6" 
                  disabled={!token || loading}
                >
                  {loading ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Guardando...
                    </>
                  ) : (
                    'Guardar y Continuar'
                  )}
                </Button>
              </form>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
