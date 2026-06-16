import { useEffect, useState } from 'react'
import { AlertCircle, Terminal, RefreshCcw, Loader2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { superadminService } from '@/services/superadmin'

export default function SuperAdminMonitoring() {
  const [errors, setErrors] = useState<any[]>([])
  const [loading, setLoading] = useState(true)

  const loadErrors = async () => {
    try {
      setLoading(true)
      const data = await superadminService.getSystemErrors()
      setErrors(data || [])
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadErrors()
  }, [])

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-foreground flex items-center gap-3">
            <Terminal className="h-8 w-8 text-purple-600" />
            Monitoreo de Errores
          </h1>
          <p className="text-muted-foreground">Log de fallos y errores del sistema detectados globalmente.</p>
        </div>
        <Button onClick={loadErrors} variant="outline" className="gap-2">
          <RefreshCcw className="h-4 w-4" />
          Actualizar
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  <th className="px-6 py-3 font-medium">Fecha</th>
                  <th className="px-6 py-3 font-medium">Tipo de Error</th>
                  <th className="px-6 py-3 font-medium">Endpoint</th>
                  <th className="px-6 py-3 font-medium">Mensaje</th>
                  <th className="px-6 py-3 font-medium">Tenant ID</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {loading ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">
                      <Loader2 className="h-6 w-6 animate-spin mx-auto mb-2" />
                      Cargando logs...
                    </td>
                  </tr>
                ) : errors.length === 0 ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-12 text-center">
                      <AlertCircle className="h-10 w-10 text-green-500 mx-auto mb-3" />
                      <p className="text-muted-foreground">Sistema saludable. No hay errores recientes.</p>
                    </td>
                  </tr>
                ) : (
                  errors.map((error, idx) => (
                    <tr key={idx} className="hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4 whitespace-nowrap text-muted-foreground">
                        {error.created_at ? new Date(error.created_at).toLocaleString() : '-'}
                      </td>
                      <td className="px-6 py-4">
                        <span className="bg-red-100 text-red-600 px-2 py-1 rounded text-xs font-semibold">
                          {error.error_type}
                        </span>
                      </td>
                      <td className="px-6 py-4 font-mono text-xs text-muted-foreground">
                        {error.method} {error.endpoint}
                      </td>
                      <td className="px-6 py-4 text-foreground">
                        {error.error_message}
                      </td>
                      <td className="px-6 py-4 text-muted-foreground text-xs font-mono">
                        {error.tenant_id || 'Global'}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
