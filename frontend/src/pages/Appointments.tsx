import { Plus, ChevronLeft, ChevronRight, MapPin, Clock, User, Check, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

const MOCK_APPOINTMENTS = [
  { id: 1, title: 'Visita - Casa San Isidro', client: 'Juan Pérez', time: '10:00 AM - 11:00 AM', status: 'Confirmada', location: 'San Isidro, BA' },
  { id: 2, title: 'Asesoramiento de Compra', client: 'María Gómez', time: '12:30 PM - 01:30 PM', status: 'Pendiente', location: 'Oficina Central / Videollamada' },
  { id: 3, title: 'Firma de Contrato', client: 'Carlos Mendoza', time: '04:00 PM - 05:00 PM', status: 'Confirmada', location: 'Oficina Central' },
]

export default function Appointments() {
  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Citas y Visitas</h1>
          <p className="text-slate-500">Administra tu agenda y las visitas a propiedades.</p>
        </div>
        <Button className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nueva Cita
        </Button>
      </div>

      <div className="flex flex-col lg:flex-row gap-6">
        {/* Calendar Sidebar (Mock) */}
        <div className="w-full lg:w-80 shrink-0 space-y-4">
          <Card>
            <div className="p-4 border-b border-slate-100 flex items-center justify-between">
              <h2 className="font-semibold text-slate-900">Mayo 2024</h2>
              <div className="flex items-center gap-1">
                <Button variant="ghost" size="icon" className="h-7 w-7"><ChevronLeft className="h-4 w-4" /></Button>
                <Button variant="ghost" size="icon" className="h-7 w-7"><ChevronRight className="h-4 w-4" /></Button>
              </div>
            </div>
            <div className="p-4">
              <div className="grid grid-cols-7 gap-1 text-center text-xs font-medium text-slate-500 mb-2">
                <div>Do</div><div>Lu</div><div>Ma</div><div>Mi</div><div>Ju</div><div>Vi</div><div>Sa</div>
              </div>
              <div className="grid grid-cols-7 gap-1 text-center text-sm">
                {/* Mock days */}
                {[...Array(31)].map((_, i) => (
                  <div 
                    key={i} 
                    className={`p-2 rounded-full cursor-pointer hover:bg-slate-100 ${
                      i === 11 ? 'bg-blue-600 text-white font-bold hover:bg-blue-700' : 'text-slate-700'
                    }`}
                  >
                    {i + 1}
                  </div>
                ))}
              </div>
            </div>
          </Card>
          
          <Card className="bg-blue-50 border-blue-100">
            <CardContent className="p-4">
              <h3 className="font-semibold text-blue-900 mb-2">Sistema de Confirmación</h3>
              <p className="text-sm text-blue-800 mb-4">
                Los clientes reciben un mensaje automático de WhatsApp 24h antes para confirmar su asistencia.
              </p>
              <Button variant="outline" className="w-full bg-white text-blue-700 border-blue-200 hover:bg-blue-50">
                Configurar Mensajes
              </Button>
            </CardContent>
          </Card>
        </div>

        {/* Daily Schedule */}
        <div className="flex-1 space-y-4">
          <div className="flex items-center justify-between mb-2">
            <h2 className="text-lg font-semibold text-slate-900">Hoy, 12 de Mayo</h2>
            <Badge variant="outline">3 Citas</Badge>
          </div>

          <div className="space-y-3">
            {MOCK_APPOINTMENTS.map((apt) => (
              <Card key={apt.id} className="overflow-hidden">
                <div className="flex flex-col sm:flex-row">
                  <div className="bg-slate-50 p-4 sm:w-48 border-b sm:border-b-0 sm:border-r border-slate-100 flex flex-col justify-center">
                    <div className="flex items-center text-slate-700 font-medium mb-1">
                      <Clock className="h-4 w-4 mr-2 text-slate-400" />
                      {apt.time.split(' - ')[0]}
                    </div>
                    <div className="text-xs text-slate-500 ml-6">
                      hasta {apt.time.split(' - ')[1]}
                    </div>
                  </div>
                  
                  <div className="p-4 flex-1 flex flex-col justify-between">
                    <div className="flex justify-between items-start mb-2">
                      <h3 className="font-semibold text-slate-900">{apt.title}</h3>
                      <Badge variant={apt.status === 'Confirmada' ? 'success' : 'warning'}>
                        {apt.status}
                      </Badge>
                    </div>
                    
                    <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-6 text-sm text-slate-600 mt-2">
                      <div className="flex items-center">
                        <User className="h-4 w-4 mr-2 text-slate-400" />
                        {apt.client}
                      </div>
                      <div className="flex items-center">
                        <MapPin className="h-4 w-4 mr-2 text-slate-400" />
                        {apt.location}
                      </div>
                    </div>
                    
                    <div className="mt-4 pt-4 border-t border-slate-100 flex gap-2">
                      <Button size="sm" variant="outline" className="text-slate-600">
                        Reprogramar
                      </Button>
                      {apt.status === 'Pendiente' && (
                        <div className="flex gap-2 ml-auto">
                          <Button size="sm" variant="outline" className="text-red-600 border-red-200 hover:bg-red-50">
                            <X className="h-4 w-4 mr-1" /> Rechazar
                          </Button>
                          <Button size="sm" className="bg-green-600 hover:bg-green-700 text-white">
                            <Check className="h-4 w-4 mr-1" /> Confirmar
                          </Button>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
