import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { FolderOpen } from 'lucide-react'

export default function Documents() {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Centro Documental</h1>
          <p className="text-muted-foreground mt-1">
            Visualiza y administra todos los documentos asociados a tus clientes, propiedades y operaciones.
          </p>
        </div>
      </div>

      <Card>
        <CardContent className="flex flex-col items-center justify-center p-12 space-y-4 text-center">
          <div className="p-4 bg-blue-50 text-blue-600 rounded-full">
            <FolderOpen className="h-12 w-12" />
          </div>
          <CardTitle>Vista Global en Desarrollo</CardTitle>
          <CardDescription className="max-w-md mx-auto">
            El listado global de documentos está en fase beta. Por el momento, puedes subir y gestionar documentos directamente desde el perfil de cada <strong>Cliente</strong>, <strong>Propiedad</strong> o <strong>Contrato</strong>.
          </CardDescription>
        </CardContent>
      </Card>
    </div>
  )
}
