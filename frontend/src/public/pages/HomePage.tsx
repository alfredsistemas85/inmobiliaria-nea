import { useEffect, useState } from 'react'
import { usePublicTenant } from '../context/PublicTenantContext'
import { Link } from 'react-router-dom'

interface FeaturedProperty {
  id: string
  title: string
  property_type: string
  operation_type: string
  price: number
  currency: string
  city: string
  main_image_url: string | null
}

export default function HomePage() {
  const { tenant } = usePublicTenant()
  const [featured, setFeatured] = useState<FeaturedProperty[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    if (!tenant) return
    const fetchFeatured = async () => {
      try {
        const res = await fetch(`/api/public/featured?tenant_id=${tenant.id}&limit=6`)
        if (res.ok) {
          const data = await res.json()
          setFeatured(data)
        }
      } catch (err) {
        console.error(err)
      } finally {
        setLoading(false)
      }
    }
    fetchFeatured()
  }, [tenant])

  return (
    <div>
      {/* Hero Section */}
      <div className="relative bg-indigo-900 text-white py-32 flex items-center justify-center text-center overflow-hidden">
        <div className="absolute inset-0 z-0 opacity-20">
          <img 
            src="https://images.unsplash.com/photo-1600596542815-ffad4c1539a9?ixlib=rb-4.0.3&auto=format&fit=crop&w=2000&q=80" 
            alt="Hero Background" 
            className="w-full h-full object-cover"
          />
        </div>
        <div className="relative z-10 max-w-3xl px-4">
          <h1 className="text-5xl font-extrabold tracking-tight mb-6 drop-shadow-md">
            Encuentra tu lugar ideal con {tenant?.name}
          </h1>
          <p className="text-xl font-light mb-10 text-indigo-100">
            Descubre las mejores propiedades en venta y alquiler adaptadas a tus necesidades.
          </p>
          <div className="flex justify-center gap-4">
            <Link to="/buscar?op=ALQUILER" className="px-8 py-4 bg-white text-indigo-900 font-bold rounded-full shadow-lg hover:bg-indigo-50 transition transform hover:-translate-y-1">
              Buscar Alquiler
            </Link>
            <Link to="/buscar?op=VENTA" className="px-8 py-4 bg-indigo-600 text-white font-bold rounded-full shadow-lg hover:bg-indigo-500 transition transform hover:-translate-y-1 border border-indigo-500">
              Buscar Venta
            </Link>
          </div>
        </div>
      </div>

      {/* Featured Properties */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        <div className="flex justify-between items-end mb-10">
          <div>
            <h2 className="text-3xl font-bold text-gray-900">Propiedades Destacadas</h2>
            <p className="text-gray-500 mt-2">Nuestra selección exclusiva para ti</p>
          </div>
          <Link to="/buscar" className="text-indigo-600 font-medium hover:text-indigo-800">Ver todas &rarr;</Link>
        </div>

        {loading ? (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
             {[1,2,3].map(i => <div key={i} className="h-80 bg-gray-200 animate-pulse rounded-2xl"></div>)}
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
            {featured.map(prop => (
              <Link to={`/propiedad/${prop.id}`} key={prop.id} className="group flex flex-col bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 border border-gray-100">
                <div className="relative h-56 overflow-hidden bg-gray-100">
                  {prop.main_image_url ? (
                    <img src={prop.main_image_url} alt={prop.title} className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
                  ) : (
                    <div className="w-full h-full flex items-center justify-center text-gray-400">Sin foto</div>
                  )}
                  <div className="absolute top-4 left-4">
                    <span className="px-3 py-1 bg-white/90 backdrop-blur-sm text-indigo-900 text-xs font-bold uppercase tracking-wider rounded-full shadow-sm">
                      {prop.operation_type}
                    </span>
                  </div>
                </div>
                <div className="p-6 flex flex-col flex-grow">
                  <div className="text-xs text-gray-500 uppercase font-semibold tracking-wide mb-1">
                    {prop.property_type} • {prop.city}
                  </div>
                  <h3 className="text-lg font-bold text-gray-900 mb-2 line-clamp-2 group-hover:text-indigo-600 transition-colors">
                    {prop.title}
                  </h3>
                  <div className="mt-auto pt-4 flex items-center justify-between border-t border-gray-50">
                    <span className="text-2xl font-black text-indigo-600">
                      {prop.currency} {Number(prop.price).toLocaleString('es-AR')}
                    </span>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
