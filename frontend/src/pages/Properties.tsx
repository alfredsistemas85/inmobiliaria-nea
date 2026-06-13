import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Plus, Search, Filter, MapPin, Bed, Bath, Maximize } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'

export const MOCK_PROPERTIES = [
  {
    id: '1',
    title: 'Casa moderna con piscina',
    location: 'San Isidro, Buenos Aires',
    price: '$450,000',
    type: 'Casa',
    status: 'Disponible',
    beds: 4,
    baths: 3,
    sqft: 250,
    image: 'https://images.unsplash.com/photo-1512917774080-9991f1c4c750?auto=format&fit=crop&q=80&w=800'
  },
  {
    id: '2',
    title: 'Departamento céntrico luminoso',
    location: 'Palermo, CABA',
    price: '$180,000',
    type: 'Departamento',
    status: 'Reservado',
    beds: 2,
    baths: 1,
    sqft: 75,
    image: 'https://images.unsplash.com/photo-1502672260266-1c1e52409818?auto=format&fit=crop&q=80&w=800'
  },
  {
    id: '3',
    title: 'Dúplex a estrenar',
    location: 'Belgrano, CABA',
    price: '$220,000',
    type: 'Dúplex',
    status: 'Disponible',
    beds: 3,
    baths: 2,
    sqft: 120,
    image: 'https://images.unsplash.com/photo-1600596542815-ffad4c1539a9?auto=format&fit=crop&q=80&w=800'
  },
]

export default function Properties() {
  const [searchTerm, setSearchTerm] = useState('')

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Propiedades</h1>
          <p className="text-slate-500">Gestiona el inventario de inmuebles.</p>
        </div>
        <Button className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nueva Propiedad
        </Button>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-400" />
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

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {MOCK_PROPERTIES.map((property) => (
          <Link key={property.id} to={`/properties/${property.id}`} className="group flex flex-col rounded-xl border border-slate-200 bg-white overflow-hidden shadow-sm hover:shadow-md transition-all">
            <div className="relative h-48 w-full overflow-hidden">
              <img 
                src={property.image} 
                alt={property.title} 
                className="h-full w-full object-cover transition-transform group-hover:scale-105"
              />
              <div className="absolute top-3 left-3">
                <Badge variant={property.status === 'Disponible' ? 'success' : 'warning'}>
                  {property.status}
                </Badge>
              </div>
              <div className="absolute top-3 right-3">
                <Badge variant="secondary" className="bg-white/90 text-slate-900 backdrop-blur-sm">
                  {property.type}
                </Badge>
              </div>
            </div>
            
            <div className="flex flex-col flex-1 p-5">
              <div className="flex justify-between items-start mb-2">
                <h3 className="font-semibold text-lg text-slate-900 line-clamp-1">{property.title}</h3>
              </div>
              <div className="flex items-center text-slate-500 text-sm mb-4">
                <MapPin className="h-4 w-4 mr-1 shrink-0" />
                <span className="truncate">{property.location}</span>
              </div>
              
              <div className="mt-auto pt-4 border-t border-slate-100 flex items-center justify-between text-slate-600 text-sm">
                <div className="flex gap-4">
                  <div className="flex items-center gap-1" title="Habitaciones">
                    <Bed className="h-4 w-4" />
                    <span>{property.beds}</span>
                  </div>
                  <div className="flex items-center gap-1" title="Baños">
                    <Bath className="h-4 w-4" />
                    <span>{property.baths}</span>
                  </div>
                  <div className="flex items-center gap-1" title="Metros Cuadrados">
                    <Maximize className="h-4 w-4" />
                    <span>{property.sqft}m²</span>
                  </div>
                </div>
                <div className="font-bold text-blue-600 text-base">
                  {property.price}
                </div>
              </div>
            </div>
          </Link>
        ))}
      </div>
    </div>
  )
}
