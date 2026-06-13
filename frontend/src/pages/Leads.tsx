import { Plus, MoreHorizontal, MessageCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

const MOCK_PIPELINE = [
  {
    id: 'new',
    title: 'Nuevos',
    leads: [
      { id: 101, name: 'Esteban Quito', source: 'WhatsApp', property: 'Casa San Isidro', time: 'Hace 2 horas' },
      { id: 102, name: 'Juana de Arco', source: 'Web', property: 'Dúplex Belgrano', time: 'Hace 5 horas' },
    ]
  },
  {
    id: 'contacted',
    title: 'Contactados',
    leads: [
      { id: 103, name: 'Pedro Pascal', source: 'Referido', property: 'Depto Palermo', time: 'Ayer' },
    ]
  },
  {
    id: 'visit',
    title: 'Visita Agendada',
    leads: [
      { id: 104, name: 'Susana Giménez', source: 'Portal Inmobiliario', property: 'Casa San Isidro', time: 'En 2 días' },
    ]
  },
  {
    id: 'negotiation',
    title: 'En Negociación',
    leads: []
  }
]

export default function Leads() {
  return (
    <div className="flex flex-col h-full space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 shrink-0">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Leads (Pipeline)</h1>
          <p className="text-slate-500">Haz seguimiento de tus oportunidades comerciales.</p>
        </div>
        <Button className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Añadir Lead
        </Button>
      </div>

      <div className="flex-1 overflow-x-auto pb-4">
        <div className="flex gap-4 h-full min-w-max">
          {MOCK_PIPELINE.map((column) => (
            <div key={column.id} className="w-[300px] flex flex-col bg-slate-100/50 rounded-xl p-3 border border-slate-200 h-full">
              <div className="flex items-center justify-between mb-4 px-1">
                <h3 className="font-semibold text-slate-700">{column.title}</h3>
                <Badge variant="secondary">{column.leads.length}</Badge>
              </div>
              
              <div className="flex-1 overflow-y-auto space-y-3">
                {column.leads.map((lead) => (
                  <Card key={lead.id} className="p-3 cursor-pointer hover:border-blue-300 transition-colors">
                    <div className="flex justify-between items-start mb-2">
                      <h4 className="font-medium text-slate-900 text-sm">{lead.name}</h4>
                      <button className="text-slate-400 hover:text-slate-700">
                        <MoreHorizontal className="h-4 w-4" />
                      </button>
                    </div>
                    <p className="text-xs text-slate-600 mb-3">{lead.property}</p>
                    
                    <div className="flex items-center justify-between mt-2 pt-2 border-t border-slate-100">
                      <span className="text-[10px] bg-slate-100 text-slate-600 px-2 py-1 rounded-md">
                        {lead.source}
                      </span>
                      <div className="flex items-center gap-2">
                        <span className="text-[10px] text-slate-500">{lead.time}</span>
                        {lead.source === 'WhatsApp' && (
                          <MessageCircle className="h-3 w-3 text-green-500" />
                        )}
                      </div>
                    </div>
                  </Card>
                ))}
                
                {column.leads.length === 0 && (
                  <div className="text-center p-4 border-2 border-dashed border-slate-200 rounded-lg text-slate-400 text-sm">
                    Sin leads
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
