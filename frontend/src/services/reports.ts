import { fetchApi } from '@/services/api';

export interface ReportFilters {
  date_from?: string
  date_to?: string
  assigned_to?: string
}

export const reportsService = {
  async downloadReport(
    type: 'leads' | 'clients' | 'properties' | 'appointments' | 'whatsapp',
    filters: ReportFilters
  ) {
    const params = new URLSearchParams()
    if (filters.date_from) params.append('date_from', filters.date_from)
    if (filters.date_to) params.append('date_to', filters.date_to)
    if (filters.assigned_to) params.append('assigned_to', filters.assigned_to)

    const token = localStorage.getItem('token')
    // VITE_API_URL apunta al host (ej. http://localhost:3000), agregamos /api explícitamente
    const apiBase = (import.meta.env.VITE_API_URL || 'http://localhost:3000').replace(/\/$/, '')
    const url = `${apiBase}/api/reports/${type}?${params.toString()}`

    const response = await fetch(url, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })

    if (!response.ok) throw new Error('Error al descargar reporte')

    const blob = await response.blob()
    const objectUrl = window.URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = objectUrl
    link.setAttribute('download', `${type}_report.csv`)
    document.body.appendChild(link)
    link.click()
    link.remove()
    window.URL.revokeObjectURL(objectUrl)
  },
}
