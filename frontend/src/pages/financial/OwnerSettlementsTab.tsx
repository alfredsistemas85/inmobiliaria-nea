import { useState, useEffect } from 'react'
import { FileText, Search, User, Download, Plus } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { fetchApi } from '@/services/api'

interface Liquidation {
  id: string
  owner_name: string
  property_title: string
  total_collected: string | number
  commission_deducted: string | number
  net_to_transfer: string | number
  period: string
  status: string
}

export function OwnerSettlementsTab() {
  const [liquidations, setLiquidations] = useState<Liquidation[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [searchTerm, setSearchTerm] = useState('')

  useEffect(() => {
    loadLiquidations()
  }, [])

  const loadLiquidations = async () => {
    try {
      setLoading(true)
      const data = await fetchApi('/financials/liquidations')
      setLiquidations(Array.isArray(data) ? data : data?.items || data?.data || [])
    } catch (err: any) {
      setError('Error al cargar liquidaciones. ' + err.message)
    } finally {
      setLoading(false)
    }
  }

  const fmt = (val: number | string | null | undefined) => {
    if (val == null) return '-'
    return `$${Number(val).toLocaleString('es-AR', { minimumFractionDigits: 2 })}`
  }

  const filteredLiquidations = liquidations.filter(l => 
    l.owner_name?.toLowerCase().includes(searchTerm.toLowerCase()) ||
    l.property_title?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  return (
    <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
      {error && (
        <div className="p-4 text-red-600 bg-red-50 dark:bg-red-900/20 rounded-md text-sm border border-red-200">
          {error}
        </div>
      )}

      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-2 border-b border-border mb-4">
          <div>
            <CardTitle className="text-xl">Liquidaciones a Propietarios</CardTitle>
            <p className="text-sm text-muted-foreground mt-1">Rendición de cobros de alquileres y deducción de honorarios.</p>
          </div>
          <div className="flex items-center gap-2">
            <div className="relative w-64">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input placeholder="Buscar propietario o propiedad..." className="pl-8" value={searchTerm} onChange={e => setSearchTerm(e.target.value)} />
            </div>
            <Button className="gap-2">
              <Plus className="h-4 w-4" /> Nueva Liquidación
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50 border-b border-border">
                <tr>
                  <th className="px-6 py-3 font-medium">Período</th>
                  <th className="px-6 py-3 font-medium">Propietario</th>
                  <th className="px-6 py-3 font-medium">Propiedad</th>
                  <th className="px-6 py-3 font-medium">Alquiler Cobrado</th>
                  <th className="px-6 py-3 font-medium">Honorarios (Deducción)</th>
                  <th className="px-6 py-3 font-medium">Neto a Transferir</th>
                  <th className="px-6 py-3 font-medium">Estado</th>
                  <th className="px-6 py-3 font-medium text-right">Acciones</th>
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr><td colSpan={8} className="px-6 py-8 text-center text-muted-foreground">Cargando liquidaciones...</td></tr>
                ) : filteredLiquidations.length === 0 ? (
                  <tr><td colSpan={8} className="px-6 py-12 text-center text-muted-foreground flex flex-col items-center gap-2">
                    <User className="h-8 w-8 opacity-20" /> No hay liquidaciones generadas.
                  </td></tr>
                ) : (
                  filteredLiquidations.map((liq, idx) => (
                    <tr key={liq.id || idx} className="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4 font-medium">{liq.period || 'Mes Actual'}</td>
                      <td className="px-6 py-4">
                        <div className="flex items-center gap-2">
                          <div className="h-6 w-6 rounded-full bg-primary/10 flex items-center justify-center text-primary text-xs font-bold">
                            {liq.owner_name?.charAt(0) || 'P'}
                          </div>
                          <span className="font-medium text-foreground">{liq.owner_name}</span>
                        </div>
                      </td>
                      <td className="px-6 py-4 text-muted-foreground">{liq.property_title}</td>
                      <td className="px-6 py-4 font-medium">{fmt(liq.total_collected)}</td>
                      <td className="px-6 py-4 text-red-600 dark:text-red-400">-{fmt(liq.commission_deducted)}</td>
                      <td className="px-6 py-4 font-bold text-emerald-600">{fmt(liq.net_to_transfer)}</td>
                      <td className="px-6 py-4">
                        <span className={`px-2.5 py-1 rounded-full text-xs font-medium ${
                          liq.status === 'PAID' 
                          ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300 border border-emerald-200' 
                          : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300 border border-yellow-200'
                        }`}>
                          {liq.status === 'PAID' ? 'Liquidado' : 'Pendiente'}
                        </span>
                      </td>
                      <td className="px-6 py-4 text-right">
                        <Button size="sm" variant="ghost" className="h-8 w-8 p-0" title="Descargar Resumen">
                          <Download className="h-4 w-4 text-muted-foreground" />
                        </Button>
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
