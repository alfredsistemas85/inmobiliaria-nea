import { Plus, Search, Filter, Phone, Mail, MoreHorizontal } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

const MOCK_CLIENTS = [
  { id: 1, name: 'Carlos Mendoza', email: 'carlos.m@example.com', phone: '+54 11 1234-5678', type: 'Propietario', status: 'Activo' },
  { id: 2, name: 'Lucía Fernández', email: 'lucia.f@example.com', phone: '+54 11 9876-5432', type: 'Comprador', status: 'En búsqueda' },
  { id: 3, name: 'Martín Rossi', email: 'martin.r@example.com', phone: '+54 11 5555-4444', type: 'Inversor', status: 'Inactivo' },
  { id: 4, name: 'Ana Silva', email: 'ana.s@example.com', phone: '+54 11 3333-2222', type: 'Inquilino', status: 'Activo' },
]

export default function Clients() {
  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Clientes</h1>
          <p className="text-slate-500">Gestiona tu cartera de clientes y propietarios.</p>
        </div>
        <Button className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nuevo Cliente
        </Button>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-400" />
          <Input 
            type="text" 
            placeholder="Buscar por nombre, email, teléfono..." 
            className="pl-9"
          />
        </div>
        <Button variant="outline" className="flex items-center gap-2">
          <Filter className="h-4 w-4" />
          Filtros
        </Button>
      </div>

      <Card className="overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left">
            <thead className="text-xs text-slate-500 uppercase bg-slate-50 border-b border-slate-200">
              <tr>
                <th className="px-6 py-4 font-medium">Nombre</th>
                <th className="px-6 py-4 font-medium">Contacto</th>
                <th className="px-6 py-4 font-medium">Tipo</th>
                <th className="px-6 py-4 font-medium">Estado</th>
                <th className="px-6 py-4 font-medium text-right">Acciones</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-200">
              {MOCK_CLIENTS.map((client) => (
                <tr key={client.id} className="bg-white hover:bg-slate-50 transition-colors">
                  <td className="px-6 py-4">
                    <div className="font-medium text-slate-900">{client.name}</div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex flex-col space-y-1">
                      <div className="flex items-center text-slate-600">
                        <Mail className="h-3 w-3 mr-2" />
                        {client.email}
                      </div>
                      <div className="flex items-center text-slate-600">
                        <Phone className="h-3 w-3 mr-2" />
                        {client.phone}
                      </div>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <Badge variant="outline">{client.type}</Badge>
                  </td>
                  <td className="px-6 py-4">
                    <Badge variant={client.status === 'Activo' ? 'success' : client.status === 'Inactivo' ? 'secondary' : 'warning'}>
                      {client.status}
                    </Badge>
                  </td>
                  <td className="px-6 py-4 text-right">
                    <Button variant="ghost" size="icon" className="h-8 w-8 text-slate-500 hover:text-slate-900">
                      <MoreHorizontal className="h-4 w-4" />
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  )
}
