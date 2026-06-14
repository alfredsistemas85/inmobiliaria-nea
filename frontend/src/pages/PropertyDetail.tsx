import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { ArrowLeft, MapPin, Bed, Bath, Maximize, Edit, Trash2, CalendarDays, CheckCircle2, Loader2, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { propertiesService } from '@/services/properties'

export default function PropertyDetail() {
  const { id } = useParams()
  const [property, setProperty] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    if (id) {
      loadProperty()
    }
  }, [id])

  const loadProperty = async () => {
    try {
      setLoading(true)
      const data = await propertiesService.getById(id!)
      setProperty(data)
    } catch (err: any) {
      setError(err.message || 'Error al cargar los detalles de la propiedad')
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px] text-muted-foreground">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    )
  }

  if (error || !property) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" size="sm" asChild>
          <Link to="/properties" className="inline-flex items-center">
            <ArrowLeft className="h-4 w-4 mr-2" /> Volver
          </Link>
        </Button>
        <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md flex items-center gap-2">
          <AlertCircle className="h-5 w-5" />
          {error || 'Propiedad no encontrada'}
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" asChild className="rounded-full">
          <Link to="/properties">
            <ArrowLeft className="h-5 w-5" />
          </Link>
        </Button>
        <div className="flex-1">
          <h1 className="text-2xl font-bold tracking-tight text-foreground">{property.title}</h1>
          <div className="flex items-center text-muted-foreground mt-1">
            <MapPin className="h-4 w-4 mr-1" />
            <span>{property.location || 'Sin ubicación'}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" className="hidden sm:flex items-center gap-2">
            <Edit className="h-4 w-4" />
            Editar
          </Button>
          <Button variant="destructive" size="sm" className="hidden sm:flex items-center gap-2">
            <Trash2 className="h-4 w-4" />
            Eliminar
          </Button>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        <div className="lg:col-span-2 space-y-6">
          <div className="overflow-hidden rounded-xl border border-border bg-muted flex items-center justify-center">
            {property.images && property.images.length > 0 ? (
              <img 
                src={property.images[0].url} 
                alt={property.title} 
                className="w-full h-[400px] object-cover"
              />
            ) : (
              <div className="h-[400px] flex items-center justify-center text-muted-foreground">
                Sin imagen disponible
              </div>
            )}
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Características Principales</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                <div className="flex flex-col items-center p-4 bg-background rounded-lg border border-border">
                  <Bed className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-foreground">{property.bedrooms || 0}</span>
                  <span className="text-xs text-muted-foreground">Habitaciones</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-background rounded-lg border border-border">
                  <Bath className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-foreground">{property.bathrooms || 0}</span>
                  <span className="text-xs text-muted-foreground">Baños</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-background rounded-lg border border-border">
                  <Maximize className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-foreground">{property.area_sqm || 0}</span>
                  <span className="text-xs text-muted-foreground">Metros²</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-background rounded-lg border border-border">
                  <CheckCircle2 className="h-6 w-6 text-green-500 mb-2" />
                  <span className="text-lg font-bold text-foreground text-center">{property.status || 'Disponible'}</span>
                  <span className="text-xs text-muted-foreground">Estado</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Descripción</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground leading-relaxed whitespace-pre-line">
                {property.description || 'Sin descripción detallada.'}
              </p>
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardContent className="p-6">
              <div className="text-sm text-muted-foreground mb-1">Precio de Venta</div>
              <div className="text-3xl font-bold text-blue-600 mb-6">
                ${property.price?.toLocaleString()}
              </div>
              
              <div className="space-y-3">
                <Button className="w-full flex items-center justify-center gap-2">
                  <CalendarDays className="h-4 w-4" />
                  Agendar Visita
                </Button>
                <Button variant="outline" className="w-full">
                  Compartir Ficha
                </Button>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Detalles Administrativos</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex justify-between py-2 border-b border-border">
                  <span className="text-muted-foreground text-sm">Tipo de Inmueble</span>
                  <span className="font-medium text-sm text-foreground">{property.property_type || 'Casa'}</span>
                </div>
                <div className="flex justify-between py-2 border-b border-border">
                  <span className="text-muted-foreground text-sm">Código</span>
                  <span className="font-medium text-sm text-foreground">REF-{property.id?.substring(0, 6).toUpperCase()}</span>
                </div>
                <div className="flex justify-between py-2 border-b border-border">
                  <span className="text-muted-foreground text-sm">Fecha de Ingreso</span>
                  <span className="font-medium text-sm text-foreground">
                    {property.created_at ? new Date(property.created_at).toLocaleDateString() : 'N/A'}
                  </span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
