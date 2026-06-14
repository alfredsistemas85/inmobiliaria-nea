import { useState, useEffect } from 'react'
import { Plus, Search, Phone, Mail, MoreHorizontal, Edit, Trash } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Card } from '@/components/ui/card'
import { Modal } from '@/components/ui/modal'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { clientsService, Client } from '@/services/clients'

const clientSchema = z.object({
  first_name: z.string().optional().nullable(),
  last_name: z.string().optional().nullable(),
  phone: z.string().min(1, 'El teléfono es obligatorio'),
  email: z.string().email('Email inválido').optional().or(z.literal('')),
  notes: z.string().optional().nullable(),
})

type ClientFormData = z.infer<typeof clientSchema>

export default function Clients() {
  const [clients, setClients] = useState<Client[]>([])
  const [loading, setLoading] = useState(true)
  const [search, setSearch] = useState('')
  const [page, setPage] = useState(1)
  const [total, setTotal] = useState(0)
  
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingClient, setEditingClient] = useState<Client | null>(null)

  const limit = 10

  const { register, handleSubmit, reset, formState: { errors } } = useForm<ClientFormData>({
    resolver: zodResolver(clientSchema),
  })

  const loadClients = async () => {
    setLoading(true)
    try {
      const response = await clientsService.getClients(limit, (page - 1) * limit, search)
      setClients(response.data)
      setTotal(response.total)
    } catch (error) {
      console.error('Error loading clients:', error)
      alert('Error al cargar clientes')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadClients()
  }, [page, search])

  const onSubmit = async (data: ClientFormData) => {
    try {
      const payload = {
        ...data,
        email: data.email || null,
      }
      
      if (editingClient) {
        await clientsService.updateClient(editingClient.id, payload)
      } else {
        await clientsService.createClient(payload)
      }
      setIsModalOpen(false)
      loadClients()
    } catch (error) {
      console.error('Error saving client:', error)
      alert('Error al guardar cliente')
    }
  }

  const handleEdit = (client: Client) => {
    setEditingClient(client)
    reset({
      first_name: client.first_name,
      last_name: client.last_name,
      phone: client.phone,
      email: client.email || '',
      notes: client.notes,
    })
    setIsModalOpen(true)
  }

  const handleDelete = async (id: string) => {
    if (confirm('¿Estás seguro de eliminar este cliente?')) {
      try {
        await clientsService.deleteClient(id)
        loadClients()
      } catch (error) {
        console.error('Error deleting client:', error)
        alert('Error al eliminar cliente')
      }
    }
  }

  const openNewClientModal = () => {
    setEditingClient(null)
    reset({ first_name: '', last_name: '', phone: '', email: '', notes: '' })
    setIsModalOpen(true)
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Clientes</h1>
          <p className="text-muted-foreground">Gestiona tu cartera de clientes y leads.</p>
        </div>
        <Button onClick={openNewClientModal} className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Nuevo Cliente
        </Button>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input 
            type="text" 
            placeholder="Buscar por nombre, email, teléfono..." 
            className="pl-9"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </div>

      <Card className="overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left">
            <thead className="text-xs text-muted-foreground uppercase bg-background border-b border-border">
              <tr>
                <th className="px-6 py-4 font-medium">Nombre</th>
                <th className="px-6 py-4 font-medium">Contacto</th>
                <th className="px-6 py-4 font-medium">Notas</th>
                <th className="px-6 py-4 font-medium text-right">Acciones</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-200">
              {loading ? (
                <tr><td colSpan={4} className="text-center py-4 text-muted-foreground">Cargando...</td></tr>
              ) : clients.length === 0 ? (
                <tr><td colSpan={4} className="text-center py-4 text-muted-foreground">No se encontraron clientes.</td></tr>
              ) : (
                clients.map((client) => (
                  <tr key={client.id} className="bg-card hover:bg-background transition-colors">
                    <td className="px-6 py-4">
                      <div className="font-medium text-foreground">{client.first_name} {client.last_name}</div>
                    </td>
                    <td className="px-6 py-4">
                      <div className="flex flex-col space-y-1">
                        {client.email && (
                          <div className="flex items-center text-muted-foreground">
                            <Mail className="h-3 w-3 mr-2" />
                            {client.email}
                          </div>
                        )}
                        <div className="flex items-center text-muted-foreground">
                          <Phone className="h-3 w-3 mr-2" />
                          {client.phone}
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 max-w-[200px] truncate text-muted-foreground">
                      {client.notes || '-'}
                    </td>
                    <td className="px-6 py-4 text-right">
                      <div className="flex justify-end gap-2">
                        <Button variant="ghost" size="icon" onClick={() => handleEdit(client)}>
                          <Edit className="h-4 w-4 text-blue-600" />
                        </Button>
                        <Button variant="ghost" size="icon" onClick={() => handleDelete(client.id)}>
                          <Trash className="h-4 w-4 text-red-600" />
                        </Button>
                      </div>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
        {/* Paginación */}
        <div className="p-4 border-t border-border flex justify-between items-center bg-background">
          <span className="text-sm text-muted-foreground">
            Mostrando {clients.length} de {total} resultados
          </span>
          <div className="flex gap-2">
            <Button 
              variant="outline" 
              size="sm" 
              disabled={page === 1}
              onClick={() => setPage(p => p - 1)}
            >
              Anterior
            </Button>
            <Button 
              variant="outline" 
              size="sm"
              disabled={page * limit >= total}
              onClick={() => setPage(p => p + 1)}
            >
              Siguiente
            </Button>
          </div>
        </div>
      </Card>

      <Modal 
        isOpen={isModalOpen} 
        onClose={() => setIsModalOpen(false)}
        title={editingClient ? 'Editar Cliente' : 'Nuevo Cliente'}
      >
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Nombre</label>
              <Input {...register('first_name')} placeholder="Juan" />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Apellido</label>
              <Input {...register('last_name')} placeholder="Pérez" />
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium text-foreground">Teléfono *</label>
            <Input {...register('phone')} placeholder="+54 11 1234-5678" />
            {errors.phone && <span className="text-xs text-red-500">{errors.phone.message}</span>}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Email</label>
            <Input type="email" {...register('email')} placeholder="juan@example.com" />
            {errors.email && <span className="text-xs text-red-500">{errors.email.message}</span>}
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Notas</label>
            <Input {...register('notes')} placeholder="Presupuesto aproximado $50.000..." />
          </div>

          <div className="pt-4 flex justify-end gap-2">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>
              Cancelar
            </Button>
            <Button type="submit">
              {editingClient ? 'Guardar Cambios' : 'Crear Cliente'}
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
