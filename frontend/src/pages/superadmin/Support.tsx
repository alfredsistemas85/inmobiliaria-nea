import { useEffect, useState } from 'react'
import { MessageSquare, Clock, CheckCircle, Search, Loader2 } from 'lucide-react'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { superadminService } from '@/services/superadmin'

export default function SuperAdminSupport() {
  const [tickets, setTickets] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [searchTerm, setSearchTerm] = useState('')

  const loadTickets = async () => {
    try {
      setLoading(true)
      const data = await superadminService.getTickets()
      setTickets(data || [])
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadTickets()
  }, [])

  const filteredTickets = (tickets || []).filter(t => 
    t.subject.toLowerCase().includes(searchTerm.toLowerCase())
  )

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-foreground flex items-center gap-3">
            <MessageSquare className="h-8 w-8 text-purple-600" />
            Soporte Técnico
          </h1>
          <p className="text-muted-foreground">Gestión de tickets de ayuda de las inmobiliarias.</p>
        </div>
      </div>

      <Card>
        <div className="p-4 border-b border-border flex flex-col sm:flex-row gap-4 justify-between">
          <div className="relative max-w-sm w-full">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input 
              type="text" 
              placeholder="Buscar por asunto..." 
              className="pl-9" 
              value={searchTerm}
              onChange={e => setSearchTerm(e.target.value)}
            />
          </div>
        </div>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  <th className="px-6 py-3 font-medium">Asunto</th>
                  <th className="px-6 py-3 font-medium">Inmobiliaria (ID)</th>
                  <th className="px-6 py-3 font-medium">Prioridad</th>
                  <th className="px-6 py-3 font-medium">Estado</th>
                  <th className="px-6 py-3 font-medium">Fecha</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {loading ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">
                      <Loader2 className="h-6 w-6 animate-spin mx-auto mb-2" />
                      Cargando tickets...
                    </td>
                  </tr>
                ) : filteredTickets.length === 0 ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-12 text-center">
                      <CheckCircle className="h-10 w-10 text-green-500 mx-auto mb-3" />
                      <p className="text-muted-foreground">No hay tickets pendientes.</p>
                    </td>
                  </tr>
                ) : (
                  filteredTickets.map((ticket, idx) => (
                    <tr key={idx} className="hover:bg-muted/50 transition-colors cursor-pointer">
                      <td className="px-6 py-4 font-medium text-foreground">
                        {ticket.subject}
                      </td>
                      <td className="px-6 py-4 text-muted-foreground font-mono text-xs">
                        {ticket.tenant_id}
                      </td>
                      <td className="px-6 py-4">
                        <Badge variant="outline" className="bg-red-50 text-red-600 border-red-200">
                          {ticket.priority || 'NORMAL'}
                        </Badge>
                      </td>
                      <td className="px-6 py-4">
                        <Badge variant="outline" className="bg-amber-50 text-amber-600 border-amber-200">
                          {ticket.status || 'OPEN'}
                        </Badge>
                      </td>
                      <td className="px-6 py-4 text-muted-foreground whitespace-nowrap">
                        <div className="flex items-center gap-2">
                          <Clock className="h-4 w-4" />
                          {ticket.created_at ? new Date(ticket.created_at).toLocaleDateString() : '-'}
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
