import { useState, useEffect } from 'react'
import { Plus, MoreHorizontal, MessageCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Modal } from '@/components/ui/modal'
import { Input } from '@/components/ui/input'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { leadsService, Lead } from '@/services/leads'
import { clientsService, Client } from '@/services/clients'
import { propertiesService, Property } from '@/services/properties'

const leadSchema = z.object({
  client_id: z.string().min(1, 'El cliente es obligatorio'),
  property_id: z.string().optional().nullable(),
  source: z.string().optional().nullable(),
  assigned_to: z.string().optional().nullable(),
})

type LeadFormData = z.infer<typeof leadSchema>

const PIPELINE_STAGES = [
  { id: 'NUEVO', title: 'Nuevos' },
  { id: 'CONTACTADO', title: 'Contactados' },
  { id: 'VISITA_AGENDADA', title: 'Visita Agendada' },
  { id: 'NEGOCIACION', title: 'En Negociación' },
  { id: 'RESERVA', title: 'Reserva' },
  { id: 'CERRADO_GANADO', title: 'Ganado' },
  { id: 'CERRADO_PERDIDO', title: 'Perdido' },
]

export default function Leads() {
  const [leads, setLeads] = useState<Lead[]>([])
  const [clients, setClients] = useState<Client[]>([])
  const [properties, setProperties] = useState<Property[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [draggedLeadId, setDraggedLeadId] = useState<string | null>(null)

  const { register, handleSubmit, reset, formState: { errors } } = useForm<LeadFormData>({
    resolver: zodResolver(leadSchema),
  })

  const loadData = async () => {
    setLoading(true)
    try {
      const [ldRes, clRes, propRes] = await Promise.all([
        leadsService.getLeads(1000, 0),
        clientsService.getClients(100, 0),
        propertiesService.getProperties(100, 0)
      ])
      setLeads(ldRes.data)
      setClients(clRes.data)
      setProperties(propRes.data)
    } catch (error) {
      console.error('Error loading data:', error)
      alert('Error al cargar leads')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
  }, [])

  const onSubmit = async (data: LeadFormData) => {
    try {
      const payload = {
        ...data,
        property_id: data.property_id || null,
        source: data.source || 'MANUAL',
        status: 'NUEVO',
      }
      
      await leadsService.createLead(payload)
      setIsModalOpen(false)
      loadData()
    } catch (error) {
      console.error('Error saving lead:', error)
      alert('Error al crear lead')
    }
  }

  const handleDragStart = (e: React.DragEvent, id: string) => {
    setDraggedLeadId(id)
    e.dataTransfer.effectAllowed = 'move'
    // Optional: make it slightly transparent while dragging
    setTimeout(() => {
      const target = e.target as HTMLElement
      if(target.style) target.style.opacity = '0.5'
    }, 0)
  }

  const handleDragEnd = (e: React.DragEvent) => {
    setDraggedLeadId(null)
    const target = e.target as HTMLElement
    if(target.style) target.style.opacity = '1'
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    e.dataTransfer.dropEffect = 'move'
  }

  const handleDrop = async (e: React.DragEvent, statusId: string) => {
    e.preventDefault()
    if (!draggedLeadId) return

    const leadToUpdate = leads.find(l => l.id === draggedLeadId)
    if (leadToUpdate && leadToUpdate.status !== statusId) {
      // Optimistic UI update
      setLeads(prev => prev.map(l => l.id === draggedLeadId ? { ...l, status: statusId } : l))

      try {
        await leadsService.updateLead(draggedLeadId, { status: statusId })
      } catch (error) {
        console.error('Error updating status:', error)
        alert('Error al actualizar estado')
        loadData() // Revert on failure
      }
    }
    setDraggedLeadId(null)
  }

  const handleConvert = async (id: string) => {
    try {
      await leadsService.convertLead(id)
      loadData()
      alert('Lead convertido exitosamente')
    } catch (error) {
      console.error('Error al convertir lead', error)
      alert('Error al convertir lead')
    }
  }

  const openNewModal = () => {
    reset({
      client_id: '',
      property_id: '',
      source: 'MANUAL',
      assigned_to: ''
    })
    setIsModalOpen(true)
  }

  const getClientName = (id: string) => {
    const c = clients.find(x => x.id === id)
    return c ? `${c.first_name || ''} ${c.last_name || ''}`.trim() : 'Cliente desconocido'
  }

  const getPropertyTitle = (id: string | null) => {
    if (!id) return 'Sin propiedad'
    const p = properties.find(x => x.id === id)
    return p ? p.title : 'Propiedad desconocida'
  }

  return (
    <div className="flex flex-col h-full space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 shrink-0">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Leads (Pipeline)</h1>
          <p className="text-slate-500">Haz seguimiento de tus oportunidades comerciales.</p>
        </div>
        <Button onClick={openNewModal} className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Añadir Lead
        </Button>
      </div>

      <div className="flex-1 overflow-x-auto pb-4">
        {loading ? (
          <p className="text-slate-500">Cargando pipeline...</p>
        ) : (
          <div className="flex gap-4 h-full min-w-max">
            {PIPELINE_STAGES.map((column) => {
              const columnLeads = leads.filter(l => (l.status || 'NUEVO') === column.id)

              return (
                <div 
                  key={column.id} 
                  className="w-[300px] flex flex-col bg-slate-100/50 rounded-xl p-3 border border-slate-200 h-full"
                  onDragOver={handleDragOver}
                  onDrop={(e) => handleDrop(e, column.id)}
                >
                  <div className="flex items-center justify-between mb-4 px-1">
                    <h3 className="font-semibold text-slate-700">{column.title}</h3>
                    <Badge variant="secondary">{columnLeads.length}</Badge>
                  </div>
                  
                  <div className="flex-1 overflow-y-auto space-y-3">
                    {columnLeads.map((lead) => {
                      const dateObj = lead.created_at ? new Date(lead.created_at) : new Date()
                      return (
                        <Card 
                          key={lead.id} 
                          className="p-3 cursor-grab active:cursor-grabbing hover:border-blue-300 transition-colors bg-white shadow-sm"
                          draggable
                          onDragStart={(e) => handleDragStart(e, lead.id)}
                          onDragEnd={handleDragEnd}
                        >
                          <div className="flex justify-between items-start mb-2">
                            <h4 className="font-medium text-slate-900 text-sm">{getClientName(lead.client_id)}</h4>
                            <button className="text-slate-400 hover:text-slate-700">
                              <MoreHorizontal className="h-4 w-4" />
                            </button>
                          </div>
                          <p className="text-xs text-slate-600 mb-3">{getPropertyTitle(lead.property_id)}</p>
                          
                          <div className="flex items-center justify-between mt-2 pt-2 border-t border-slate-100">
                            <span className="text-[10px] bg-slate-100 text-slate-600 px-2 py-1 rounded-md">
                              {lead.source || 'MANUAL'}
                            </span>
                            <div className="flex items-center gap-2">
                              {lead.status === 'RESERVA' && (
                                <button 
                                  onClick={() => handleConvert(lead.id)}
                                  className="text-[10px] bg-green-100 text-green-700 px-2 py-1 rounded-md hover:bg-green-200 transition-colors cursor-pointer"
                                  title="Convertir Lead"
                                >
                                  Convertir
                                </button>
                              )}
                              <span className="text-[10px] text-slate-500">
                                {dateObj.toLocaleDateString()}
                              </span>
                            </div>
                          </div>
                        </Card>
                      )
                    })}
                    
                    {columnLeads.length === 0 && (
                      <div className="text-center p-4 border-2 border-dashed border-slate-200 rounded-lg text-slate-400 text-sm bg-transparent pointer-events-none">
                        Arrastra un lead aquí
                      </div>
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      <Modal 
        isOpen={isModalOpen} 
        onClose={() => setIsModalOpen(false)}
        title="Nuevo Lead"
      >
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Cliente *</label>
            <select 
              {...register('client_id')}
              className="flex h-10 w-full rounded-md border border-slate-200 bg-white px-3 py-2 text-sm ring-offset-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950"
            >
              <option value="">Seleccione un cliente</option>
              {clients.map(c => (
                <option key={c.id} value={c.id}>{c.first_name} {c.last_name} ({c.email || c.phone})</option>
              ))}
            </select>
            {errors.client_id && <span className="text-xs text-red-500">{errors.client_id.message}</span>}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Propiedad de Interés</label>
            <select 
              {...register('property_id')}
              className="flex h-10 w-full rounded-md border border-slate-200 bg-white px-3 py-2 text-sm ring-offset-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950"
            >
              <option value="">(Ninguna)</option>
              {properties.map(p => (
                <option key={p.id} value={p.id}>{p.title}</option>
              ))}
            </select>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Origen (Source)</label>
            <Input {...register('source')} placeholder="WhatsApp, Web, Referido..." defaultValue="MANUAL" />
          </div>

          <div className="pt-4 flex justify-end gap-2">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>
              Cancelar
            </Button>
            <Button type="submit">
              Crear Lead
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
