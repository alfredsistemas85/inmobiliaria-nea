import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Plus, Search, Filter, MapPin, Bed, Bath, Maximize, AlertCircle, Loader2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { propertiesService } from '@/services/properties'

export default function Properties() {
  const [searchTerm, setSearchTerm] = useState('')
  const [properties, setProperties] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  useEffect(() => {
    loadProperties()
  }, [])

  const loadProperties = async () => {
    try {
      setLoading(true)
      const data = await propertiesService.getAll()
      setProperties(data || [])
    } catch (err: any) {
      setError(err.message || 'Error al cargar propiedades')
    } finally {
      setLoading(false)
    }
  }

  const filteredProperties = properties.filter(p => 
    p.title?.toLowerCase().includes(searchTerm.toLowerCase()) || 
    p.location?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Propiedades</h1>
          <p className="text-muted-foreground">Gestiona el inventario de inmuebles.</p>
        </div>
        <Button className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nueva Propiedad
        </Button>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input 
            type="text" 
            placeholder="Buscar por título, ubicación..." 
            className="pl-9"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
        <Button variant="outline" className="flex items-center gap-2">
          <Filter className="h-4 w-4" />
          Filtros
        </Button>
      </div>

      {loading && (
        <div className="flex items-center justify-center p-12 text-muted-foreground">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      )}

      {error && (
        <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md flex items-center gap-2">
          <AlertCircle className="h-5 w-5" />
          {error}
        </div>
      )}

      {!loading && !error && filteredProperties.length === 0 && (
        <div className="text-center p-12 text-muted-foreground border border-border rounded-xl bg-background border-dashed">
          No se encontraron propiedades.
        </div>
      )}

      {!loading && !error && filteredProperties.length > 0 && (
        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {filteredProperties.map((property) => (
            <Link key={property.id} to={`/properties/${property.id}`} className="group flex flex-col rounded-xl border border-border bg-card overflow-hidden shadow-sm hover:shadow-md transition-all">
              <div className="relative h-48 w-full overflow-hidden bg-muted">
                {property.images && property.images.length > 0 ? (
                  <img 
                    src={property.images[0].url} 
                    alt={property.title} 
                    className="h-full w-full object-cover transition-transform group-hover:scale-105"
                  />
                ) : (
                  <div className="h-full w-full flex items-center justify-center text-muted-foreground">
                    Sin imagen
                  </div>
                )}
                <div className="absolute top-3 left-3">
                  <Badge variant={property.status === 'Disponible' ? 'success' : 'warning'}>
                    {property.status || 'Disponible'}
                  </Badge>
                </div>
                <div className="absolute top-3 right-3">
                  <Badge variant="secondary" className="bg-card/90 text-foreground backdrop-blur-sm">
                    {property.property_type || 'Casa'}
                  </Badge>
                </div>
              </div>
              
              <div className="flex flex-col flex-1 p-5">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-semibold text-lg text-foreground line-clamp-1">{property.title}</h3>
                </div>
                <div className="flex items-center text-muted-foreground text-sm mb-4">
                  <MapPin className="h-4 w-4 mr-1 shrink-0" />
                  <span className="truncate">{property.location || 'Sin ubicación'}</span>
                </div>
                
                <div className="mt-auto pt-4 border-t border-border flex items-center justify-between text-muted-foreground text-sm">
                  <div className="flex gap-4">
                    <div className="flex items-center gap-1" title="Habitaciones">
                      <Bed className="h-4 w-4" />
                      <span>{property.bedrooms || 0}</span>
                    </div>
                    <div className="flex items-center gap-1" title="Baños">
                      <Bath className="h-4 w-4" />
                      <span>{property.bathrooms || 0}</span>
                    </div>
                    <div className="flex items-center gap-1" title="Metros Cuadrados">
                      <Maximize className="h-4 w-4" />
                      <span>{property.area_sqm || 0}m²</span>
                    </div>
                  </div>
                  <div className="font-bold text-blue-600 text-base">
                    ${property.price?.toLocaleString()}
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
