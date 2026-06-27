import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { propertiesService } from '@/services/properties'
import { clientsService, Client } from '@/services/clients'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Loader2, ArrowLeft, Upload, Plus, Trash2 } from 'lucide-react'

export default function PropertyForm() {
  const { id } = useParams()
  const navigate = useNavigate()
  
  const [loading, setLoading] = useState(false)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  const [formData, setFormData] = useState({
    title: '',
    description: '',
    property_type: 'Casa',
    operation_type: 'Venta',
    price: 0,
    currency: 'USD',
    address: '',
    city: '',
    province: '',
    square_meters: 0,
    bedrooms: 0,
    bathrooms: 0,
    status: 'Disponible',
    owners: [] as { client_id: string; percentage: number }[]
  })

  const [clients, setClients] = useState<Client[]>([])

  // Archivos
  const [images, setImages] = useState<FileList | null>(null)
  const [documents, setDocuments] = useState<FileList | null>(null)

  useEffect(() => {
    clientsService.getClients(100).then(res => setClients(res.data || []))
    if (id) {
      loadProperty()
    }
  }, [id])

  const loadProperty = async () => {
    setLoading(true)
    try {
      const data = await propertiesService.getById(id!)
      setFormData({
        title: data.title || '',
        description: data.description || '',
        property_type: data.property_type || 'Casa',
        operation_type: data.operation_type || 'Venta',
        price: data.price || 0,
        currency: data.currency || 'USD',
        address: data.address || '',
        city: data.city || '',
        province: data.province || '',
        square_meters: data.square_meters || 0,
        bedrooms: data.bedrooms || 0,
        bathrooms: data.bathrooms || 0,
        status: data.status || 'Disponible',
        owners: data.owners || []
      })
    } catch (err: any) {
      setError(err.message || 'Error al cargar propiedad')
    } finally {
      setLoading(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError('')
    try {
      let propertyId = id;
      // 1. Guardar o actualizar la propiedad base
      if (id) {
        await propertiesService.update(id, formData)
      } else {
        const created = await propertiesService.create(formData)
        propertyId = created.id
      }

      // 2. Subir imágenes si se seleccionaron
      if (images && images.length > 0) {
        for (let i = 0; i < images.length; i++) {
          await propertiesService.uploadImage(propertyId!, images[i])
        }
      }

      // 3. Subir documentos si se seleccionaron
      if (documents && documents.length > 0) {
        for (let i = 0; i < documents.length; i++) {
          await propertiesService.uploadDocument(propertyId!, documents[i])
        }
      }

      navigate(`/properties/${propertyId}`)
    } catch (err: any) {
      setError(err.message || 'Error al guardar la propiedad')
    } finally {
      setSaving(false)
    }
  }

  if (loading) {
    return <div className="flex justify-center p-12"><Loader2 className="h-8 w-8 animate-spin text-muted-foreground" /></div>
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate(-1)} className="rounded-full">
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">
            {id ? 'Editar Propiedad' : 'Nueva Propiedad'}
          </h1>
          <p className="text-muted-foreground">
            Complete los datos básicos de la propiedad e incluya archivos.
          </p>
        </div>
      </div>

      {error && (
        <div className="p-4 text-red-600 bg-red-50 border border-red-100 rounded-md">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-6">
        <Card>
          <CardHeader>
            <CardTitle>Datos Principales</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Título</label>
              <Input 
                required 
                value={formData.title} 
                onChange={e => setFormData({...formData, title: e.target.value})} 
                placeholder="Ej: Casa 3 ambientes en Palermo"
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Descripción</label>
              <textarea 
                required
                rows={4}
                className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                value={formData.description} 
                onChange={e => setFormData({...formData, description: e.target.value})}
              />
            </div>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Tipo</label>
                <select 
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  value={formData.property_type}
                  onChange={e => setFormData({...formData, property_type: e.target.value})}
                >
                  <option>Casa</option>
                  <option>Departamento</option>
                  <option>Terreno</option>
                  <option>Local</option>
                  <option>Oficina</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Operación</label>
                <select 
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  value={formData.operation_type}
                  onChange={e => setFormData({...formData, operation_type: e.target.value})}
                >
                  <option>Venta</option>
                  <option>Alquiler</option>
                  <option>Alquiler Temporal</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Precio</label>
                <Input type="number" min="0" required value={formData.price} onChange={e => setFormData({...formData, price: Number(e.target.value)})} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Moneda</label>
                <select 
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  value={formData.currency}
                  onChange={e => setFormData({...formData, currency: e.target.value})}
                >
                  <option>USD</option>
                  <option>ARS</option>
                </select>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Ubicación y Dimensiones</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Dirección</label>
                <Input value={formData.address} onChange={e => setFormData({...formData, address: e.target.value})} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Ciudad</label>
                <Input required value={formData.city} onChange={e => setFormData({...formData, city: e.target.value})} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Provincia</label>
                <Input required value={formData.province} onChange={e => setFormData({...formData, province: e.target.value})} />
              </div>
            </div>
            
            <div className="grid grid-cols-3 gap-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Metros Totales</label>
                <Input type="number" min="0" value={formData.square_meters} onChange={e => setFormData({...formData, square_meters: Number(e.target.value)})} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Habitaciones</label>
                <Input type="number" min="0" value={formData.bedrooms} onChange={e => setFormData({...formData, bedrooms: Number(e.target.value)})} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Baños</label>
                <Input type="number" min="0" value={formData.bathrooms} onChange={e => setFormData({...formData, bathrooms: Number(e.target.value)})} />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between">
            <CardTitle>Propietarios</CardTitle>
            <Button type="button" size="sm" variant="outline" onClick={() => setFormData({...formData, owners: [...formData.owners, { client_id: '', percentage: 100 }]})}>
              <Plus className="h-4 w-4 mr-1" /> Añadir Propietario
            </Button>
          </CardHeader>
          <CardContent className="space-y-4">
            {formData.owners.map((owner, i) => (
              <div key={i} className="flex gap-4 items-end bg-muted p-4 rounded-lg">
                <div className="flex-1">
                  <label className="text-sm font-medium">Cliente</label>
                  <select
                    required
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    value={owner.client_id}
                    onChange={e => {
                      const newOwners = [...formData.owners]
                      newOwners[i].client_id = e.target.value
                      setFormData({...formData, owners: newOwners})
                    }}
                  >
                    <option value="">Seleccionar propietario...</option>
                    {clients.map(c => (
                      <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                    ))}
                  </select>
                </div>
                <div className="w-32">
                  <label className="text-sm font-medium">% Propiedad</label>
                  <Input 
                    type="number" 
                    min="0" 
                    max="100" 
                    required 
                    value={owner.percentage}
                    onChange={e => {
                      const newOwners = [...formData.owners]
                      newOwners[i].percentage = Number(e.target.value)
                      setFormData({...formData, owners: newOwners})
                    }}
                  />
                </div>
                <Button type="button" variant="ghost" className="text-red-500" onClick={() => {
                  const newOwners = [...formData.owners]
                  newOwners.splice(i, 1)
                  setFormData({...formData, owners: newOwners})
                }}>
                  <Trash2 className="h-4 w-4" />
                </Button>
              </div>
            ))}
            {formData.owners.length === 0 && (
              <p className="text-sm text-muted-foreground">No hay propietarios asignados.</p>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Archivos</CardTitle>
            <CardDescription>Formatos de imagen (10MB): jpg, png, webp. Documentos (20MB): pdf, doc, xlsx.</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium flex items-center gap-2">
                <Upload className="h-4 w-4" /> Imágenes
              </label>
              <Input 
                type="file" 
                multiple 
                accept=".jpg,.jpeg,.png,.webp"
                onChange={(e) => setImages(e.target.files)}
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium flex items-center gap-2">
                <Upload className="h-4 w-4" /> Documentos Privados
              </label>
              <Input 
                type="file" 
                multiple 
                accept=".pdf,.doc,.docx,.xlsx"
                onChange={(e) => setDocuments(e.target.files)}
              />
            </div>
          </CardContent>
        </Card>

        <div className="flex justify-end gap-4">
          <Button type="button" variant="outline" onClick={() => navigate(-1)} disabled={saving}>Cancelar</Button>
          <Button type="submit" disabled={saving}>
            {saving ? <><Loader2 className="mr-2 h-4 w-4 animate-spin" /> Guardando...</> : 'Guardar Propiedad'}
          </Button>
        </div>
      </form>
    </div>
  )
}
