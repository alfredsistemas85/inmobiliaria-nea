import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { reportsService, ReportFilters } from '@/services/reports'
import { Download, Users, Home, CalendarDays, MessageCircle, FileText } from 'lucide-react'

export default function Reports() {
  const [filters, setFilters] = useState<ReportFilters>({})
  const [loading, setLoading] = useState<string | null>(null)

  const handleDownload = async (type: 'leads' | 'clients' | 'properties' | 'appointments' | 'whatsapp') => {
    setLoading(type)
    try {
      await reportsService.downloadReport(type, filters)
    } catch (error) {
      console.error('Error downloading report', error)
      alert('Error descargando el reporte')
    } finally {
      setLoading(null)
    }
  }

  const REPORTS = [
    { id: 'clients', title: 'Clientes', icon: Users, description: 'Exporta todos los clientes registrados y su información de contacto.' },
    { id: 'properties', title: 'Propiedades', icon: Home, description: 'Catálogo completo de propiedades, su estado y precio.' },
    { id: 'leads', title: 'Leads', icon: FileText, description: 'Oportunidades de negocio, estados y fuentes de ingreso.' },
    { id: 'appointments', title: 'Citas', icon: CalendarDays, description: 'Citas programadas, historial y asistencia.' },
    { id: 'whatsapp', title: 'WhatsApp', icon: MessageCircle, description: 'Historial de conversaciones y actividad por agente.' },
  ] as const

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight text-foreground">Reportes y Exportaciones</h1>
        <p className="text-muted-foreground">Descarga la información de tu cuenta en formato CSV.</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Filtros Globales</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <div className="space-y-1">
              <label className="text-sm font-medium">Fecha Desde</label>
              <input 
                type="date" 
                className="w-full rounded-md border border-border px-3 py-2 text-sm"
                onChange={(e) => setFilters(f => ({ ...f, date_from: e.target.value ? new Date(e.target.value).toISOString() : undefined }))}
              />
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium">Fecha Hasta</label>
              <input 
                type="date" 
                className="w-full rounded-md border border-border px-3 py-2 text-sm"
                onChange={(e) => setFilters(f => ({ ...f, date_to: e.target.value ? new Date(e.target.value).toISOString() : undefined }))}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {REPORTS.map((report) => (
          <Card key={report.id}>
            <CardHeader className="flex flex-row items-center gap-4 space-y-0 pb-2">
              <div className="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                <report.icon className="h-5 w-5 text-blue-600" />
              </div>
              <div className="flex-1">
                <CardTitle className="text-base">{report.title}</CardTitle>
              </div>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground mb-4 h-10">
                {report.description}
              </p>
              <button
                onClick={() => handleDownload(report.id)}
                disabled={loading === report.id}
                className="w-full flex items-center justify-center gap-2 bg-slate-900 text-white rounded-md py-2 text-sm font-medium hover:bg-slate-800 disabled:opacity-50"
              >
                <Download className="h-4 w-4" />
                {loading === report.id ? 'Descargando...' : 'Descargar CSV'}
              </button>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
