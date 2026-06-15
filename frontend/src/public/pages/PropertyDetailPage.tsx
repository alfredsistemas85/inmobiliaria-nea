import { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import { usePublicTenant } from '../context/PublicTenantContext'

export default function PropertyDetailPage() {
  const { id } = useParams()
  const { tenant, portal } = usePublicTenant()
  const [property, setProperty] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  
  const [formMsg, setFormMsg] = useState('')

  useEffect(() => {
    if (!tenant || !id) return
    const fetchProp = async () => {
      try {
        const res = await fetch(`/api/public/properties/${id}?tenant_id=${tenant.id}`)
        if (!res.ok) throw new Error('No se encontró la propiedad')
        const data = await res.json()
        setProperty(data)
      } catch (err: any) {
        setError(err.message)
      } finally {
        setLoading(false)
      }
    }
    fetchProp()
  }, [id, tenant])

  const submitLead = async (e: React.FormEvent) => {
    e.preventDefault()
    const form = e.target as HTMLFormElement
    const formData = new FormData(form)
    
    try {
      const res = await fetch('/api/public/leads', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tenant_id: tenant?.id,
          property_id: id,
          name: formData.get('name'),
          phone: formData.get('phone'),
          email: formData.get('email'),
          message: formData.get('message'),
          website: formData.get('website') // honeypot
        })
      })
      if (res.ok) {
        setFormMsg('¡Consulta enviada exitosamente!')
        form.reset()
      } else if (res.status === 429) {
        setFormMsg('Has enviado demasiadas consultas. Intenta más tarde.')
      } else {
        setFormMsg('Ocurrió un error. Intenta nuevamente.')
      }
    } catch {
      setFormMsg('Ocurrió un error de red.')
    }
  }

  if (loading) return <div className="h-screen flex items-center justify-center">Cargando...</div>
  if (error || !property) return <div className="h-screen flex items-center justify-center flex-col"><h1 className="text-2xl font-bold">Propiedad no encontrada</h1></div>

  return (
    <div className="bg-white min-h-screen">
      {/* Header Info */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-10 pb-6">
        <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 mb-6">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <span className="px-3 py-1 bg-indigo-100 text-indigo-800 text-xs font-bold uppercase tracking-wider rounded-full">
                {property.operation_type}
              </span>
              <span className="text-sm font-semibold text-gray-500 uppercase tracking-wider">
                {property.property_type}
              </span>
            </div>
            <h1 className="text-3xl md:text-5xl font-extrabold text-gray-900 tracking-tight leading-tight">
              {property.title}
            </h1>
            <p className="text-gray-500 text-lg mt-2 flex items-center gap-2">
              📍 {property.address}, {property.city}, {property.province}
            </p>
          </div>
          <div className="text-left md:text-right">
             <div className="text-4xl font-black text-indigo-600">
               {property.currency} {Number(property.price).toLocaleString('es-AR')}
             </div>
          </div>
        </div>
      </div>

      {/* Galería Visual */}
      <div className="bg-gray-100 border-y border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
           {property.images && property.images.length > 0 ? (
             <div className="grid grid-cols-1 md:grid-cols-2 gap-4 h-[500px]">
               <div className="h-full">
                 <img src={property.images[0].url} className="w-full h-full object-cover rounded-l-2xl" />
               </div>
               <div className="grid grid-cols-2 gap-4 h-full">
                 {property.images.slice(1, 5).map((img: any, i: number) => (
                   <img key={i} src={img.url} className={`w-full h-full object-cover ${i === 1 ? 'rounded-tr-2xl' : ''} ${i === 3 ? 'rounded-br-2xl' : ''}`} />
                 ))}
               </div>
             </div>
           ) : (
             <div className="h-[400px] flex items-center justify-center bg-gray-200 rounded-2xl text-gray-500 text-lg font-medium">
               Sin imágenes
             </div>
           )}
        </div>
      </div>

      {/* Contenido Principal */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="flex flex-col lg:flex-row gap-16">
          <div className="w-full lg:w-2/3">
            <h2 className="text-2xl font-bold text-gray-900 mb-6">Características Principales</h2>
            <div className="flex gap-8 mb-12 py-6 border-y border-gray-100">
               <div className="text-center">
                 <div className="text-3xl font-black text-gray-900">{property.square_meters}</div>
                 <div className="text-sm font-medium text-gray-500 uppercase tracking-wide mt-1">m² totales</div>
               </div>
               <div className="text-center">
                 <div className="text-3xl font-black text-gray-900">{property.bedrooms}</div>
                 <div className="text-sm font-medium text-gray-500 uppercase tracking-wide mt-1">Habitaciones</div>
               </div>
               <div className="text-center">
                 <div className="text-3xl font-black text-gray-900">{property.bathrooms}</div>
                 <div className="text-sm font-medium text-gray-500 uppercase tracking-wide mt-1">Baños</div>
               </div>
            </div>

            <h2 className="text-2xl font-bold text-gray-900 mb-6">Descripción</h2>
            <div className="prose max-w-none text-gray-600 text-lg leading-relaxed mb-12 whitespace-pre-wrap">
              {property.description}
            </div>
          </div>

          {/* Formulario / Contacto */}
          <div className="w-full lg:w-1/3">
            <div className="bg-white rounded-3xl p-8 border border-gray-100 shadow-xl sticky top-24">
              <h3 className="text-xl font-bold text-gray-900 mb-2">¿Te interesa?</h3>
              <p className="text-gray-500 mb-6">Completa el formulario y nos contactaremos contigo a la brevedad.</p>
              
              <form onSubmit={submitLead} className="flex flex-col gap-4">
                {/* Honeypot */}
                <input type="text" name="website" className="hidden" tabIndex={-1} autoComplete="off" />
                
                <div>
                  <input required type="text" name="name" placeholder="Tu nombre completo" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
                </div>
                <div>
                  <input required type="text" name="phone" placeholder="Tu teléfono" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
                </div>
                <div>
                  <input type="email" name="email" placeholder="Tu email (opcional)" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
                </div>
                <div>
                  <textarea required name="message" rows={4} placeholder="Mensaje" defaultValue={`Hola, me interesa la propiedad: ${property.title}.`} className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3"></textarea>
                </div>
                <button type="submit" className="w-full bg-indigo-600 text-white font-bold py-4 rounded-xl hover:bg-indigo-700 transition shadow-md hover:shadow-lg">
                  Enviar Consulta
                </button>
                {formMsg && <div className="text-center mt-2 text-sm font-medium text-green-600 bg-green-50 py-2 rounded-lg">{formMsg}</div>}
              </form>

              {portal?.allow_whatsapp && tenant?.whatsapp && (
                <div className="mt-6 pt-6 border-t border-gray-100">
                  <a href={`https://wa.me/${tenant.whatsapp.replace(/\D/g,'')}?text=Hola,%20me%20interesa%20la%20propiedad%20${encodeURIComponent(property.title)}`} target="_blank" rel="noreferrer" className="w-full flex items-center justify-center gap-2 bg-[#25D366] text-white font-bold py-4 rounded-xl hover:bg-[#20bd5a] transition shadow-md hover:shadow-lg">
                    <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.888-.788-1.489-1.761-1.663-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"/></svg>
                    Contactar por WhatsApp
                  </a>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
