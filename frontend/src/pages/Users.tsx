import { useState, useEffect } from 'react'
import { usersService, User } from '@/services/users'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { UserPlus, Edit, Trash2, Shield, Search } from 'lucide-react'

// Roles según la Fase 3
const ROLES = [
  { id: 'ADMIN_INMOBILIARIA', label: 'Administrador' },
  { id: 'SUPERVISOR', label: 'Supervisor' },
  { id: 'AGENTE', label: 'Agente' },
  { id: 'OPERADOR', label: 'Operador' }
];

export default function Users() {
  const [users, setUsers] = useState<User[]>([])
  const [loading, setLoading] = useState(true)
  const [search, setSearch] = useState('')

  // Formularios
  const [showForm, setShowForm] = useState(false)
  const [editingUser, setEditingUser] = useState<User | null>(null)
  
  const [formData, setFormData] = useState({
    first_name: '',
    last_name: '',
    email: '',
    password: '',
    role: ROLES[2].id, // Default to AGENTE
    is_active: true
  })

  const loadUsers = async () => {
    setLoading(true)
    try {
      const data = await usersService.getUsers()
      setUsers(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadUsers()
  }, [])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      if (editingUser) {
        await usersService.updateUser(editingUser.id, {
          first_name: formData.first_name,
          last_name: formData.last_name,
          email: formData.email,
          role: formData.role,
          is_active: formData.is_active
        })
      } else {
        await usersService.createUser({
          first_name: formData.first_name,
          last_name: formData.last_name,
          email: formData.email,
          password: formData.password || '123456', // Pass por defecto si no ingresan
          role: formData.role
        })
      }
      setShowForm(false)
      setEditingUser(null)
      loadUsers()
    } catch (err) {
      alert("Error al guardar usuario. Verifique los datos o si el email ya existe.");
    }
  }

  const handleDelete = async (id: string) => {
    if (confirm('¿Estás seguro de desactivar (eliminar) a este usuario?')) {
      try {
        await usersService.deleteUser(id)
        loadUsers()
      } catch (err) {
        console.error(err)
      }
    }
  }

  const openEdit = (user: User) => {
    setEditingUser(user)
    setFormData({
      first_name: user.first_name || '',
      last_name: user.last_name || '',
      email: user.email,
      password: '',
      role: user.role || ROLES[2].id,
      is_active: user.is_active !== false
    })
    setShowForm(true)
  }

  const openCreate = () => {
    setEditingUser(null)
    setFormData({
      first_name: '',
      last_name: '',
      email: '',
      password: '',
      role: ROLES[2].id,
      is_active: true
    })
    setShowForm(true)
  }

  const filteredUsers = users.filter(u => 
    (u.first_name?.toLowerCase() || '').includes(search.toLowerCase()) || 
    (u.last_name?.toLowerCase() || '').includes(search.toLowerCase()) || 
    (u.email?.toLowerCase() || '').includes(search.toLowerCase())
  )

  const getRoleLabel = (roleIdOrName: string) => {
    const r = ROLES.find(r => r.id === roleIdOrName)
    return r ? r.label : roleIdOrName
  }

  if (showForm) {
    return (
      <div className="max-w-2xl mx-auto space-y-6">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold tracking-tight text-foreground">
            {editingUser ? 'Editar Usuario' : 'Nuevo Usuario'}
          </h1>
          <Button variant="outline" onClick={() => setShowForm(false)}>Cancelar</Button>
        </div>
        <Card>
          <CardContent className="pt-6">
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Nombre</label>
                  <Input required value={formData.first_name} onChange={e => setFormData({...formData, first_name: e.target.value})} />
                </div>
                <div className="space-y-2">
                  <label className="text-sm font-medium">Apellido</label>
                  <Input required value={formData.last_name} onChange={e => setFormData({...formData, last_name: e.target.value})} />
                </div>
              </div>
              
              <div className="space-y-2">
                <label className="text-sm font-medium">Email</label>
                <Input required type="email" value={formData.email} onChange={e => setFormData({...formData, email: e.target.value})} />
              </div>

              {!editingUser && (
                <div className="space-y-2">
                  <label className="text-sm font-medium">Contraseña Temporal</label>
                  <Input required type="password" value={formData.password} onChange={e => setFormData({...formData, password: e.target.value})} />
                </div>
              )}

              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Rol</label>
                  <select 
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                    value={formData.role}
                    onChange={e => setFormData({...formData, role: e.target.value})}
                  >
                    {ROLES.map(r => (
                      <option key={r.id} value={r.id}>{r.label}</option>
                    ))}
                  </select>
                </div>
                
                {editingUser && (
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Estado</label>
                    <select 
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                      value={formData.is_active ? 'true' : 'false'}
                      onChange={e => setFormData({...formData, is_active: e.target.value === 'true'})}
                    >
                      <option value="true">Activo</option>
                      <option value="false">Inactivo</option>
                    </select>
                  </div>
                )}
              </div>

              <div className="pt-4 flex justify-end gap-2">
                <Button type="button" variant="outline" onClick={() => setShowForm(false)}>Cancelar</Button>
                <Button type="submit">Guardar Usuario</Button>
              </div>
            </form>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Gestión de Usuarios</h1>
          <p className="text-muted-foreground">Administra los accesos y roles de tu agencia.</p>
        </div>
        <Button onClick={openCreate} className="w-full sm:w-auto">
          <UserPlus className="mr-2 h-4 w-4" /> Nuevo Usuario
        </Button>
      </div>

      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Equipo Inmobiliario</CardTitle>
              <CardDescription>Usuarios activos e inactivos del sistema.</CardDescription>
            </div>
            <div className="relative w-64">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input 
                placeholder="Buscar por nombre o email..." 
                className="pl-8"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>
          </div>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="py-8 text-center text-muted-foreground">Cargando usuarios...</div>
          ) : (
            <div className="rounded-md border">
              <table className="w-full text-sm text-left">
                <thead className="bg-muted/50 text-muted-foreground">
                  <tr>
                    <th className="px-4 py-3 font-medium">Nombre</th>
                    <th className="px-4 py-3 font-medium">Email</th>
                    <th className="px-4 py-3 font-medium">Rol</th>
                    <th className="px-4 py-3 font-medium">Estado</th>
                    <th className="px-4 py-3 font-medium text-right">Acciones</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-border">
                  {filteredUsers.length === 0 ? (
                    <tr>
                      <td colSpan={5} className="px-4 py-8 text-center text-muted-foreground">
                        No se encontraron usuarios
                      </td>
                    </tr>
                  ) : (
                    filteredUsers.map((user) => (
                      <tr key={user.id} className="hover:bg-muted/50">
                        <td className="px-4 py-3">
                          <div className="font-medium text-foreground">{user.first_name} {user.last_name}</div>
                        </td>
                        <td className="px-4 py-3">{user.email}</td>
                        <td className="px-4 py-3">
                          <div className="flex items-center gap-1.5">
                            <Shield className="h-3.5 w-3.5 text-blue-500" />
                            {getRoleLabel(user.role || '')}
                          </div>
                        </td>
                        <td className="px-4 py-3">
                          <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                            user.is_active !== false ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
                          }`}>
                            {user.is_active !== false ? 'Activo' : 'Inactivo'}
                          </span>
                        </td>
                        <td className="px-4 py-3 text-right">
                          <div className="flex items-center justify-end gap-2">
                            <Button variant="ghost" size="sm" onClick={() => openEdit(user)}>
                              <Edit className="h-4 w-4 text-blue-600" />
                            </Button>
                            <Button variant="ghost" size="sm" onClick={() => handleDelete(user.id)}>
                              <Trash2 className="h-4 w-4 text-red-600" />
                            </Button>
                          </div>
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
