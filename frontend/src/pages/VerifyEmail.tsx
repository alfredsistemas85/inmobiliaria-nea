import { useEffect, useState } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'
import { CheckCircle, XCircle, Loader2 } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { authService } from '@/services/auth'

export default function VerifyEmail() {
  const [searchParams] = useSearchParams()
  const token = searchParams.get('token')
  const navigate = useNavigate()
  
  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading')
  const [errorMessage, setErrorMessage] = useState('')

  useEffect(() => {
    if (!token) {
      setStatus('error')
      setErrorMessage('No se proporcionó ningún token de verificación.')
      return
    }

    const verify = async () => {
      try {
        await authService.verifyEmail(token)
        setStatus('success')
      } catch (err: any) {
        setStatus('error')
        setErrorMessage(err.message || 'El enlace es inválido o ha expirado.')
      }
    }

    verify()
  }, [token])

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <Card className="mt-8 text-center">
          <CardHeader>
            <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-blue-50">
              {status === 'loading' && <Loader2 className="h-8 w-8 animate-spin text-blue-600" />}
              {status === 'success' && <CheckCircle className="h-8 w-8 text-green-600" />}
              {status === 'error' && <XCircle className="h-8 w-8 text-red-600" />}
            </div>
            <CardTitle>
              {status === 'loading' && 'Verificando cuenta...'}
              {status === 'success' && '¡Cuenta verificada!'}
              {status === 'error' && 'Error de verificación'}
            </CardTitle>
            <CardDescription>
              {status === 'loading' && 'Por favor espera mientras validamos tu información.'}
              {status === 'success' && 'Tu dirección de correo ha sido confirmada exitosamente. Ya puedes acceder al sistema.'}
              {status === 'error' && errorMessage}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {/* Espacio adicional si se requiere contenido */}
          </CardContent>
          <CardFooter className="flex justify-center">
            {status !== 'loading' && (
              <Button onClick={() => navigate('/login')} className="w-full sm:w-auto">
                Ir al Inicio de Sesión
              </Button>
            )}
          </CardFooter>
        </Card>
      </div>
    </div>
  )
}
