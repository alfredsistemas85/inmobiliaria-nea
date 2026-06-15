import { useState } from 'react'
import { usePublicTenant } from '../context/PublicTenantContext'

export default function ContactPage() {
  const { tenant } = usePublicTenant()
  const [formMsg, setFormMsg] = useState('')

  const submitContact = async (e: React.FormEvent) => {
    e.preventDefault()
    const form = e.target as HTMLFormElement
    const formData = new FormData(form)
    
    try {
      const res = await fetch('/api/public/leads', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tenant_id: tenant?.id,
          name: formData.get('name'),
          phone: formData.get('phone'),
          email: formData.get('email'),
          message: formData.get('message'),
          website: formData.get('website') // honeypot
        })
      })
      if (res.ok) {
        setFormMsg('¡Mensaje enviado exitosamente!')
        form.reset()
      } else {
        setFormMsg('Ocurrió un error. Intenta nuevamente.')
      }
    } catch {
      setFormMsg('Ocurrió un error de red.')
    }
  }

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-extrabold text-gray-900 tracking-tight">Contacto</h1>
        <p className="text-lg text-gray-500 mt-4">Estamos aquí para ayudarte a encontrar lo que buscas.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-12 bg-white rounded-3xl p-8 sm:p-12 shadow-xl border border-gray-100">
        <div>
          <h2 className="text-2xl font-bold text-gray-900 mb-6">Nuestra Oficina</h2>
          <div className="space-y-6 text-gray-600">
             {tenant?.address && (
               <div className="flex items-start gap-4">
                 <div className="text-indigo-600 mt-1">📍</div>
                 <div>
                   <div className="font-bold text-gray-900">Dirección</div>
                   <div>{tenant.address}</div>
                 </div>
               </div>
             )}
             {tenant?.phone && (
               <div className="flex items-start gap-4">
                 <div className="text-indigo-600 mt-1">📞</div>
                 <div>
                   <div className="font-bold text-gray-900">Teléfono</div>
                   <div>{tenant.phone}</div>
                 </div>
               </div>
             )}
             {tenant?.email && (
               <div className="flex items-start gap-4">
                 <div className="text-indigo-600 mt-1">✉️</div>
                 <div>
                   <div className="font-bold text-gray-900">Email</div>
                   <div>{tenant.email}</div>
                 </div>
               </div>
             )}
          </div>
        </div>

        <div>
          <form onSubmit={submitContact} className="flex flex-col gap-4">
            <input type="text" name="website" className="hidden" tabIndex={-1} autoComplete="off" />
            
            <div>
              <input required type="text" name="name" placeholder="Tu nombre completo" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
            </div>
            <div>
              <input required type="text" name="phone" placeholder="Tu teléfono" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
            </div>
            <div>
              <input type="email" name="email" placeholder="Tu email" className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3" />
            </div>
            <div>
              <textarea required name="message" rows={5} placeholder="Tu mensaje..." className="w-full rounded-xl border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 px-4 py-3"></textarea>
            </div>
            <button type="submit" className="w-full bg-indigo-600 text-white font-bold py-4 rounded-xl hover:bg-indigo-700 transition shadow-md">
              Enviar Mensaje
            </button>
            {formMsg && <div className="text-center mt-2 text-sm font-medium text-green-600 bg-green-50 py-2 rounded-lg">{formMsg}</div>}
          </form>
        </div>
      </div>
    </div>
  )
}
