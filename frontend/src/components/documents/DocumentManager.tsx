import { useState, useEffect } from 'react'
import { FileText, Download, Trash2, Loader2, UploadCloud, Image as ImageIcon } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

interface DocumentManagerProps {
  entityType: 'client' | 'property' | 'contract' | 'lead'
  entityId: string
  title?: string
}

export default function DocumentManager({ entityType, entityId, title = "Documentos Adjuntos" }: DocumentManagerProps) {
  const [documents, setDocuments] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [uploading, setUploading] = useState(false)

  useEffect(() => {
    loadDocuments()
  }, [entityId])

  const loadDocuments = async () => {
    try {
      setLoading(true)
      const token = localStorage.getItem('token')
      const res = await fetch(`${import.meta.env.VITE_API_URL}/api/documents/entity/${entityType}/${entityId}`, {
        headers: { 'Authorization': `Bearer ${token}` }
      })
      if (res.ok) {
        const data = await res.json()
        setDocuments(data)
      }
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;
    const file = e.target.files[0]

    try {
      setUploading(true)
      const token = localStorage.getItem('token')
      
      // 1. Get Presigned URL
      const res = await fetch(`${import.meta.env.VITE_API_URL}/api/documents/upload-url`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          file_name: file.name,
          file_size: file.size,
          mime_type: file.type,
          entity_type: entityType,
          entity_id: entityId
        })
      })

      if (!res.ok) throw new Error("No se pudo generar la URL de subida")
      const { upload_url } = await res.json()

      // 2. Upload file directly to Supabase Storage
      const uploadRes = await fetch(upload_url, {
        method: 'PUT',
        headers: {
          'Content-Type': file.type,
        },
        body: file
      })

      if (!uploadRes.ok) throw new Error("Fallo al subir archivo a S3/Supabase")

      // Reload
      await loadDocuments()
    } catch (err) {
      console.error(err)
      alert("Error al subir el documento")
    } finally {
      setUploading(false)
      // reset input
      e.target.value = ''
    }
  }

  const handleDownload = async (docId: string) => {
    try {
      const token = localStorage.getItem('token')
      const res = await fetch(`${import.meta.env.VITE_API_URL}/api/documents/${docId}`, {
        headers: { 'Authorization': `Bearer ${token}` }
      })
      if (!res.ok) throw new Error()
      
      const { url } = await res.json()
      window.open(url, '_blank')
    } catch (err) {
      console.error(err)
      alert("Error al obtener el enlace de descarga")
    }
  }

  const handleDelete = async (docId: string) => {
    if (!confirm("¿Seguro que deseas eliminar este documento?")) return;
    try {
      const token = localStorage.getItem('token')
      const res = await fetch(`${import.meta.env.VITE_API_URL}/api/documents/${docId}`, {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${token}` }
      })
      if (!res.ok) throw new Error()
      
      await loadDocuments()
    } catch (err) {
      console.error(err)
      alert("Error al eliminar documento")
    }
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle>{title}</CardTitle>
        <div>
          <input 
            type="file" 
            id={`file-upload-${entityId}`} 
            className="hidden" 
            onChange={handleFileUpload}
            disabled={uploading}
            accept=".pdf,.doc,.docx,.xls,.xlsx,.png,.jpg,.jpeg"
          />
          <label htmlFor={`file-upload-${entityId}`}>
            <Button variant="outline" size="sm" className="cursor-pointer" disabled={uploading} type="button" onClick={() => document.getElementById(`file-upload-${entityId}`)?.click()}>
              {uploading ? <Loader2 className="h-4 w-4 mr-2 animate-spin" /> : <UploadCloud className="h-4 w-4 mr-2" />}
              Subir Documento
            </Button>
          </label>
        </div>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="flex justify-center p-4"><Loader2 className="h-6 w-6 animate-spin text-muted-foreground" /></div>
        ) : documents.length === 0 ? (
          <div className="text-center p-6 text-muted-foreground bg-muted/50 rounded-lg border border-dashed">
            No hay documentos adjuntos.
          </div>
        ) : (
          <div className="space-y-3">
            {documents.map((doc: any) => (
              <div key={doc.id} className="flex items-center justify-between p-3 border rounded-lg bg-card hover:bg-accent/50 transition-colors">
                <div className="flex items-center gap-3 overflow-hidden">
                  <div className="p-2 bg-blue-50 text-blue-600 rounded-md shrink-0">
                    {doc.mime_type.includes('image') ? <ImageIcon className="h-5 w-5" /> : <FileText className="h-5 w-5" />}
                  </div>
                  <div className="truncate">
                    <p className="font-medium text-sm text-foreground truncate" title={doc.file_name}>{doc.file_name}</p>
                    <div className="flex items-center gap-2 text-xs text-muted-foreground mt-1">
                      <span>{(doc.file_size / 1024 / 1024).toFixed(2)} MB</span>
                      <span>•</span>
                      <span>{new Date(doc.created_at).toLocaleDateString()}</span>
                      {doc.version > 1 && <Badge variant="secondary" className="text-[10px] px-1 py-0 h-4">v{doc.version}</Badge>}
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-1 shrink-0 ml-4">
                  <Button variant="ghost" size="icon" onClick={() => handleDownload(doc.id)} title="Descargar / Ver">
                    <Download className="h-4 w-4 text-muted-foreground" />
                  </Button>
                  <Button variant="ghost" size="icon" onClick={() => handleDelete(doc.id)} className="hover:text-red-600">
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
