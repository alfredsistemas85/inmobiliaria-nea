import { useEffect, useState } from 'react'
import { LifeBuoy, MessageSquare, Clock } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

export default function SuperAdminSupport() {
  const [tickets, setTickets] = useState<any[]>([])

  useEffect(() => {
    // Mock tickets
    setTickets([
      { id: 1, subject: 'Problema con conexión de WhatsApp', tenant: 'Inmobiliaria Central', status: 'OPEN', priority: 'HIGH', updated_at: 'hace 30 min' },
      { id: 2, subject: 'Duda sobre facturación', tenant: 'Propiedades del Norte', status: 'IN_PROGRESS', priority: 'NORMAL', updated_at: 'hace 2 horas' },
      { id: 3, subject: 'No puedo agregar usuarios', tenant: 'Sur Bienes Raíces', status: 'RESOLVED', priority: 'NORMAL', updated_at: 'hace 1 día' },
    ])
  }, [])

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-foreground">Soporte Técnico</h1>
          <p className="text-muted-foreground">Gestión de tickets de ayuda de las inmobiliarias.</p>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Tickets Abiertos</CardTitle>
            <LifeBuoy className="h-4 w-4 text-blue-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-500">8</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">En Progreso</CardTitle>
            <Clock className="h-4 w-4 text-amber-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-amber-500">3</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Resueltos (Semana)</CardTitle>
            <MessageSquare className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-500">24</div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  <th className="px-6 py-3 font-medium">Asunto</th>
                  <th className="px-6 py-3 font-medium">Tenant</th>
                  <th className="px-6 py-3 font-medium">Estado</th>
                  <th className="px-6 py-3 font-medium">Prioridad</th>
                  <th className="px-6 py-3 font-medium">Última Actividad</th>
                  <th className="px-6 py-3 text-right font-medium">Acción</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {tickets.map(ticket => (
                  <tr key={ticket.id} className="hover:bg-muted/50 transition-colors">
                    <td className="px-6 py-4 font-medium text-foreground">{ticket.subject}</td>
                    <td className="px-6 py-4 text-muted-foreground">{ticket.tenant}</td>
                    <td className="px-6 py-4">
                      <Badge variant="outline" className={
                        ticket.status === 'OPEN' ? "border-blue-500 text-blue-500" :
                        ticket.status === 'IN_PROGRESS' ? "border-amber-500 text-amber-500" :
                        "border-green-500 text-green-500"
                      }>
                        {ticket.status}
                      </Badge>
                    </td>
                    <td className="px-6 py-4">
                      <Badge variant="secondary" className={
                        ticket.priority === 'HIGH' ? "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400" : ""
                      }>
                        {ticket.priority}
                      </Badge>
                    </td>
                    <td className="px-6 py-4 text-muted-foreground">{ticket.updated_at}</td>
                    <td className="px-6 py-4 text-right">
                      <Button variant="ghost" size="sm" className="text-purple-600 hover:text-purple-700">Ver Ticket</Button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
