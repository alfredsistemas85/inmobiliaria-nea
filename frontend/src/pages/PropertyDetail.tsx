import { useParams, Link } from 'react-router-dom'
import { ArrowLeft, MapPin, Bed, Bath, Maximize, Edit, Trash2, CalendarDays, CheckCircle2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { MOCK_PROPERTIES } from './Properties'

export default function PropertyDetail() {
  const { id } = useParams()
  const property = MOCK_PROPERTIES.find(p => p.id === id) || MOCK_PROPERTIES[0]

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" asChild className="rounded-full">
          <Link to="/properties">
            <ArrowLeft className="h-5 w-5" />
          </Link>
        </Button>
        <div className="flex-1">
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">{property.title}</h1>
          <div className="flex items-center text-slate-500 mt-1">
            <MapPin className="h-4 w-4 mr-1" />
            <span>{property.location}</span>
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
          <div className="overflow-hidden rounded-xl border border-slate-200">
            <img 
              src={property.image} 
              alt={property.title} 
              className="w-full h-[400px] object-cover"
            />
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Características Principales</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                <div className="flex flex-col items-center p-4 bg-slate-50 rounded-lg border border-slate-100">
                  <Bed className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-slate-900">{property.beds}</span>
                  <span className="text-xs text-slate-500">Habitaciones</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-slate-50 rounded-lg border border-slate-100">
                  <Bath className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-slate-900">{property.baths}</span>
                  <span className="text-xs text-slate-500">Baños</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-slate-50 rounded-lg border border-slate-100">
                  <Maximize className="h-6 w-6 text-blue-600 mb-2" />
                  <span className="text-2xl font-bold text-slate-900">{property.sqft}</span>
                  <span className="text-xs text-slate-500">Metros²</span>
                </div>
                <div className="flex flex-col items-center p-4 bg-slate-50 rounded-lg border border-slate-100">
                  <CheckCircle2 className="h-6 w-6 text-green-500 mb-2" />
                  <span className="text-lg font-bold text-slate-900 text-center">{property.status}</span>
                  <span className="text-xs text-slate-500">Estado</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Descripción</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-slate-600 leading-relaxed">
                Excelente propiedad ubicada en una de las mejores zonas. Cuenta con amplios ambientes muy luminosos.
                Ideal para familias que buscan comodidad y tranquilidad. Cercano a colegios, supermercados y transporte público.
                <br /><br />
                La propiedad dispone de un amplio living comedor, cocina integrada totalmente equipada, habitaciones con placares empotrados y un hermoso jardín con piscina.
              </p>
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardContent className="p-6">
              <div className="text-sm text-slate-500 mb-1">Precio de Venta</div>
              <div className="text-3xl font-bold text-blue-600 mb-6">{property.price}</div>
              
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
                <div className="flex justify-between py-2 border-b border-slate-100">
                  <span className="text-slate-500 text-sm">Tipo de Inmueble</span>
                  <span className="font-medium text-sm text-slate-900">{property.type}</span>
                </div>
                <div className="flex justify-between py-2 border-b border-slate-100">
                  <span className="text-slate-500 text-sm">Código</span>
                  <span className="font-medium text-sm text-slate-900">REF-{property.id}A89</span>
                </div>
                <div className="flex justify-between py-2 border-b border-slate-100">
                  <span className="text-slate-500 text-sm">Propietario</span>
                  <span className="font-medium text-sm text-blue-600">Carlos Mendoza</span>
                </div>
                <div className="flex justify-between py-2 border-b border-slate-100">
                  <span className="text-slate-500 text-sm">Fecha de Ingreso</span>
                  <span className="font-medium text-sm text-slate-900">12 May 2024</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
