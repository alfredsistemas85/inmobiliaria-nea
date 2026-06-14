import { useState, useEffect } from 'react'
import { Plus, ChevronLeft, ChevronRight, MapPin, Clock, User, Check, X, Calendar } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Modal } from '@/components/ui/modal'
import { Input } from '@/components/ui/input'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { appointmentsService, Appointment } from '@/services/appointments'
import { clientsService, Client } from '@/services/clients'
import { propertiesService, Property } from '@/services/properties'

const appointmentSchema = z.object({
  client_id: z.string().min(1, 'El cliente es obligatorio'),
  property_id: z.string().optional().nullable(),
  scheduled_at: z.string().min(1, 'La fecha y hora son obligatorias'),
  status: z.string().optional().nullable(),
  notes: z.string().optional().nullable(),
})

type AppointmentFormData = z.infer<typeof appointmentSchema>

export default function Appointments() {
  const [appointments, setAppointments] = useState<Appointment[]>([])
  const [clients, setClients] = useState<Client[]>([])
  const [properties, setProperties] = useState<Property[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)

  const { register, handleSubmit, reset, formState: { errors } } = useForm<AppointmentFormData>({
    resolver: zodResolver(appointmentSchema),
  })

  const loadData = async () => {
    setLoading(true)
    try {
      const [aptRes, clRes, propRes] = await Promise.all([
        appointmentsService.getAppointments(50, 0),
        clientsService.getClients(100, 0),
        propertiesService.getProperties(100, 0)
      ])
      setAppointments(aptRes.data)
      setClients(clRes.data)
      setProperties(propRes.data)
    } catch (error) {
      console.error('Error loading data:', error)
      alert('Error al cargar citas')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadData()
  }, [])

  const onSubmit = async (data: AppointmentFormData) => {
    try {
      // Ensure date is ISO
      const isoDate = new Date(data.scheduled_at).toISOString()
      const payload = {
        ...data,
        scheduled_at: isoDate,
        property_id: data.property_id || null,
        status: data.status || 'Pendiente'
      }
      
      await appointmentsService.createAppointment(payload)
      setIsModalOpen(false)
      loadData()
    } catch (error) {
      console.error('Error saving appointment:', error)
      alert('Error al guardar la cita')
    }
  }

  const handleStatusChange = async (id: string, status: string) => {
    try {
      await appointmentsService.updateAppointment(id, { status })
      loadData()
    } catch (error) {
      console.error('Error updating status:', error)
      alert('Error al actualizar estado')
    }
  }

  const openNewModal = () => {
    reset({
      client_id: '',
      property_id: '',
      scheduled_at: '',
      status: 'Pendiente',
      notes: ''
    })
    setIsModalOpen(true)
  }

  const getClientName = (id: string) => {
    const c = clients.find(x => x.id === id)
    return c ? `${c.first_name || ''} ${c.last_name || ''}`.trim() : 'Cliente desconocido'
  }

  const getPropertyTitle = (id: string | null) => {
    if (!id) return 'Sin propiedad asignada'
    const p = properties.find(x => x.id === id)
    return p ? p.title : 'Propiedad desconocida'
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Citas y Visitas</h1>
          <p className="text-slate-500">Administra tu agenda y las visitas a propiedades.</p>
        </div>
        <Button onClick={openNewModal} className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nueva Cita
        </Button>
      </div>

      <div className="flex flex-col lg:flex-row gap-6">
        {/* Calendar Sidebar */}
        <div className="w-full lg:w-80 shrink-0 space-y-4">
          <Card className="bg-blue-50 border-blue-100">
            <CardContent className="p-4">
              <h3 className="font-semibold text-blue-900 mb-2">Sistema de Confirmación</h3>
              <p className="text-sm text-blue-800 mb-4">
                Próximamente: Integración con WhatsApp para recordatorios automáticos.
              </p>
            </CardContent>
          </Card>
        </div>

        {/* Schedule */}
        <div className="flex-1 space-y-4">
          <div className="flex items-center justify-between mb-2">
            <h2 className="text-lg font-semibold text-slate-900">Próximas Citas</h2>
            <Badge variant="outline">{appointments.length} Citas</Badge>
          </div>

          <div className="space-y-3">
            {loading ? (
              <p className="text-slate-500">Cargando citas...</p>
            ) : appointments.length === 0 ? (
              <p className="text-slate-500">No hay citas programadas.</p>
            ) : (
              appointments.map((apt) => {
                const dateObj = new Date(apt.scheduled_at)
                const dateStr = dateObj.toLocaleDateString()
                const timeStr = dateObj.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })

                return (
                  <Card key={apt.id} className="overflow-hidden">
                    <div className="flex flex-col sm:flex-row">
                      <div className="bg-slate-50 p-4 sm:w-48 border-b sm:border-b-0 sm:border-r border-slate-100 flex flex-col justify-center">
                        <div className="flex items-center text-slate-700 font-medium mb-1">
                          <Calendar className="h-4 w-4 mr-2 text-slate-400" />
                          {dateStr}
                        </div>
                        <div className="flex items-center text-xs text-slate-500 ml-6">
                          <Clock className="h-3 w-3 mr-1" />
                          {timeStr}
                        </div>
                      </div>
                      
                      <div className="p-4 flex-1 flex flex-col justify-between">
                        <div className="flex justify-between items-start mb-2">
                          <h3 className="font-semibold text-slate-900">{getPropertyTitle(apt.property_id)}</h3>
                          <Badge variant={apt.status === 'Confirmada' ? 'success' : apt.status === 'Cancelada' ? 'destructive' : 'warning'}>
                            {apt.status || 'Pendiente'}
                          </Badge>
                        </div>
                        
                        <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-6 text-sm text-slate-600 mt-2">
                          <div className="flex items-center">
                            <User className="h-4 w-4 mr-2 text-slate-400" />
                            {getClientName(apt.client_id)}
                          </div>
                        </div>
                        
                        {apt.notes && (
                          <div className="mt-2 text-sm text-slate-500 italic">
                            "{apt.notes}"
                          </div>
                        )}

                        <div className="mt-4 pt-4 border-t border-slate-100 flex gap-2 justify-end">
                          {(!apt.status || apt.status === 'Pendiente') && (
                            <>
                              <Button size="sm" variant="outline" className="text-red-600 border-red-200 hover:bg-red-50" onClick={() => handleStatusChange(apt.id, 'Cancelada')}>
                                <X className="h-4 w-4 mr-1" /> Cancelar
                              </Button>
                              <Button size="sm" className="bg-green-600 hover:bg-green-700 text-white" onClick={() => handleStatusChange(apt.id, 'Confirmada')}>
                                <Check className="h-4 w-4 mr-1" /> Confirmar
                              </Button>
                            </>
                          )}
                        </div>
                      </div>
                    </div>
                  </Card>
                )
              })
            )}
          </div>
        </div>
      </div>

      <Modal 
        isOpen={isModalOpen} 
        onClose={() => setIsModalOpen(false)}
        title="Nueva Cita"
      >
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Cliente *</label>
            <select 
              {...register('client_id')}
              className="flex h-10 w-full rounded-md border border-slate-200 bg-white px-3 py-2 text-sm ring-offset-white file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-slate-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="">Seleccione un cliente</option>
              {clients.map(c => (
                <option key={c.id} value={c.id}>{c.first_name} {c.last_name} ({c.email || c.phone})</option>
              ))}
            </select>
            {errors.client_id && <span className="text-xs text-red-500">{errors.client_id.message}</span>}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Propiedad</label>
            <select 
              {...register('property_id')}
              className="flex h-10 w-full rounded-md border border-slate-200 bg-white px-3 py-2 text-sm ring-offset-white file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-slate-500 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="">(Ninguna)</option>
              {properties.map(p => (
                <option key={p.id} value={p.id}>{p.title}</option>
              ))}
            </select>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Fecha y Hora *</label>
            <Input type="datetime-local" {...register('scheduled_at')} />
            {errors.scheduled_at && <span className="text-xs text-red-500">{errors.scheduled_at.message}</span>}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Notas</label>
            <Input {...register('notes')} placeholder="Detalles de la cita..." />
          </div>

          <div className="pt-4 flex justify-end gap-2">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>
              Cancelar
            </Button>
            <Button type="submit">
              Guardar Cita
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
