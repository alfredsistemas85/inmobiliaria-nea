import { useEffect, useState } from 'react'
import { usePublicTenant } from '../context/PublicTenantContext'
import { Link, useSearchParams } from 'react-router-dom'

interface PropertyItem {
  id: string
  title: string
  property_type: string
  operation_type: string
  price: number
  currency: string
  city: string
  bedrooms: number
  bathrooms: number
  main_image_url: string | null
}

export default function PropertyListPage() {
  const { tenant } = usePublicTenant()
  const [searchParams, setSearchParams] = useSearchParams()
  
  const [properties, setProperties] = useState<PropertyItem[]>([])
  const [total, setTotal] = useState(0)
  const [loading, setLoading] = useState(true)

  // Filters state
  const op = searchParams.get('op') || ''
  const type = searchParams.get('type') || ''
  const city = searchParams.get('city') || ''
  const pmin = searchParams.get('pmin') || ''
  const pmax = searchParams.get('pmax') || ''

  useEffect(() => {
    if (!tenant) return
    const fetchProps = async () => {
      setLoading(true)
      try {
        const query = new URLSearchParams()
        query.set('tenant_id', tenant.id)
        if (op) query.set('operation_type', op)
        if (type) query.set('property_type', type)
        if (city) query.set('city', city)
        if (pmin) query.set('price_min', pmin)
        if (pmax) query.set('price_max', pmax)

        const res = await fetch(`/api/public/properties?${query.toString()}`)
        if (res.ok) {
          const data = await res.json()
          setProperties(data.data)
          setTotal(data.total)
        }
      } catch (err) {
        console.error(err)
      } finally {
        setLoading(false)
      }
    }
    fetchProps()
  }, [tenant, op, type, city, pmin, pmax])

  const handleFilter = (e: React.FormEvent) => {
    e.preventDefault()
    const form = e.target as HTMLFormElement
    const formData = new FormData(form)
    
    const params = new URLSearchParams()
    for (const [key, val] of Array.from(formData.entries())) {
      if (val) params.set(key, val as string)
    }
    setSearchParams(params)
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 flex flex-col md:flex-row gap-8">
      {/* Sidebar Filters */}
      <div className="w-full md:w-64 flex-shrink-0">
        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 sticky top-24">
          <h3 className="font-bold text-gray-900 mb-4 text-lg">Filtros</h3>
          <form onSubmit={handleFilter} className="flex flex-col gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Operación</label>
              <select name="op" defaultValue={op} className="w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500">
                <option value="">Todas</option>
                <option value="VENTA">Venta</option>
                <option value="ALQUILER">Alquiler</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Tipo</label>
              <select name="type" defaultValue={type} className="w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500">
                <option value="">Todos</option>
                <option value="CASA">Casa</option>
                <option value="DEPARTAMENTO">Departamento</option>
                <option value="TERRENO">Terreno</option>
                <option value="LOCAL">Local</option>
                <option value="OFICINA">Oficina</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Ciudad</label>
              <input type="text" name="city" defaultValue={city} placeholder="Ej. Corrientes" className="w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500" />
            </div>
            <div className="flex gap-2">
              <div className="w-1/2">
                <label className="block text-xs font-medium text-gray-500 mb-1">Precio Min</label>
                <input type="number" name="pmin" defaultValue={pmin} className="w-full border-gray-300 rounded-md shadow-sm text-sm" />
              </div>
              <div className="w-1/2">
                <label className="block text-xs font-medium text-gray-500 mb-1">Precio Max</label>
                <input type="number" name="pmax" defaultValue={pmax} className="w-full border-gray-300 rounded-md shadow-sm text-sm" />
              </div>
            </div>
            <button type="submit" className="mt-4 w-full bg-indigo-600 text-white font-bold py-2 px-4 rounded-md hover:bg-indigo-700 transition">
              Aplicar Filtros
            </button>
            {(op || type || city || pmin || pmax) && (
              <button type="button" onClick={() => setSearchParams({})} className="text-sm text-gray-500 hover:text-gray-900 mt-2 text-center w-full">
                Limpiar Filtros
              </button>
            )}
          </form>
        </div>
      </div>

      {/* Grid */}
      <div className="flex-grow">
        <div className="mb-6">
          <h1 className="text-3xl font-bold text-gray-900">Propiedades</h1>
          <p className="text-gray-500 mt-1">{total} resultados encontrados</p>
        </div>

        {loading ? (
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-8">
             {[1,2,3,4].map(i => <div key={i} className="h-80 bg-gray-200 animate-pulse rounded-2xl"></div>)}
          </div>
        ) : properties.length === 0 ? (
          <div className="bg-white p-12 text-center rounded-2xl border border-gray-100">
             <div className="text-gray-400 mb-4">
               <svg className="w-16 h-16 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                 <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
               </svg>
             </div>
             <h3 className="text-lg font-bold text-gray-900">No se encontraron propiedades</h3>
             <p className="text-gray-500 mt-2">Prueba ajustando los filtros de búsqueda.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-8">
            {properties.map(prop => (
              <Link to={`/propiedad/${prop.id}`} key={prop.id} className="group flex flex-col bg-white rounded-2xl overflow-hidden shadow-sm hover:shadow-xl transition-all duration-300 border border-gray-100">
                <div className="relative h-60 overflow-hidden bg-gray-100">
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
                  <div className="flex gap-4 text-sm text-gray-500 mb-4">
                    {prop.bedrooms > 0 && <span>🛏 {prop.bedrooms} Hab.</span>}
                    {prop.bathrooms > 0 && <span>🚿 {prop.bathrooms} Baños</span>}
                  </div>
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
