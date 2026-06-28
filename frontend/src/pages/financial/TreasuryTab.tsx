import { useState, useEffect } from 'react'
import { Wallet, ArrowUpRight, ArrowDownRight, Plus, Search, Building2, Download } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Modal } from '@/components/ui/modal'
import { fetchApi } from '@/services/api'

interface Movement {
  id: string
  amount: number | string
  movement_type: 'IN' | 'OUT'
  description: string
  created_at: string
  reference_id: string | null
}

interface TreasurySummary {
  balance: number
  total_in: number
  total_out: number
}

export function TreasuryTab() {
  const [movements, setMovements] = useState<Movement[]>([])
  const [summary, setSummary] = useState<TreasurySummary>({ balance: 0, total_in: 0, total_out: 0 })
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  // Modal
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [saving, setSaving] = useState(false)
  const [form, setForm] = useState({
    amount: '',
    movement_type: 'IN',
    description: '',
  })

  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    try {
      setLoading(true)
      const data = await fetchApi('/financials/treasury/movements')
      // Note: Endpoint may need to return { movements: [], summary: {} } or similar
      if (Array.isArray(data)) {
        setMovements(data)
        // Calculate basic summary if backend doesn't provide it
        const totals = data.reduce((acc, curr) => {
          const amt = Number(curr.amount)
          if (curr.movement_type === 'IN') {
             acc.total_in += amt
             acc.balance += amt
          } else {
             acc.total_out += amt
             acc.balance -= amt
          }
          return acc
        }, { balance: 0, total_in: 0, total_out: 0 })
        setSummary(totals)
      } else {
        setMovements(data.movements || [])
        if (data.summary) setSummary(data.summary)
      }
    } catch (err: any) {
      setError('Error al cargar tesorería. ' + err.message)
    } finally {
      setLoading(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    try {
      await fetchApi('/financials/treasury/movements', {
        method: 'POST',
        body: JSON.stringify({
          amount: parseFloat(form.amount),
          movement_type: form.movement_type,
          description: form.description,
        })
      })
      setIsModalOpen(false)
      loadData()
    } catch (err: any) {
      alert('Error: ' + err.message)
    } finally {
      setSaving(false)
    }
  }

  const fmt = (val: number | string) => `$${Number(val).toLocaleString('es-AR', { minimumFractionDigits: 2 })}`

  return (
    <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
      
      {/* Resumen Cards */}
      <div className="grid gap-4 md:grid-cols-3">
        <Card className="bg-primary/5 border-primary/10">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Saldo Actual (Caja)</CardTitle>
            <Wallet className="h-4 w-4 text-primary" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-primary">{fmt(summary.balance)}</div>
            <p className="text-xs text-muted-foreground mt-1">Disponible operativo</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Ingresos Totales</CardTitle>
            <ArrowUpRight className="h-4 w-4 text-emerald-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-emerald-600">{fmt(summary.total_in)}</div>
            <p className="text-xs text-muted-foreground mt-1">Acumulado</p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Egresos Totales</CardTitle>
            <ArrowDownRight className="h-4 w-4 text-red-600" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">{fmt(summary.total_out)}</div>
            <p className="text-xs text-muted-foreground mt-1">Acumulado</p>
          </CardContent>
        </Card>
      </div>

      {/* Tabla de Movimientos */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-2 border-b border-border mb-4">
          <CardTitle>Historial de Movimientos</CardTitle>
          <Button className="gap-2" onClick={() => setIsModalOpen(true)}>
            <Plus className="h-4 w-4" /> Nuevo Movimiento
          </Button>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50 border-b border-border">
                <tr>
                  <th className="px-6 py-3 font-medium">Fecha</th>
                  <th className="px-6 py-3 font-medium">Tipo</th>
                  <th className="px-6 py-3 font-medium">Descripción</th>
                  <th className="px-6 py-3 font-medium">Monto</th>
                  <th className="px-6 py-3 font-medium text-right">Recibo</th>
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr><td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">Cargando caja...</td></tr>
                ) : movements.length === 0 ? (
                  <tr><td colSpan={5} className="px-6 py-12 text-center text-muted-foreground flex flex-col items-center gap-2">
                    <Wallet className="h-8 w-8 opacity-20" /> No hay movimientos en caja.
                  </td></tr>
                ) : (
                  movements.map((mov) => (
                    <tr key={mov.id} className="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4 text-muted-foreground">
                        {new Date(mov.created_at).toLocaleString('es-AR', { dateStyle: 'short', timeStyle: 'short' })}
                      </td>
                      <td className="px-6 py-4">
                        {mov.movement_type === 'IN' ? (
                          <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300 border border-emerald-200">
                            <ArrowUpRight className="h-3 w-3" /> Ingreso
                          </span>
                        ) : (
                          <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300 border border-red-200">
                            <ArrowDownRight className="h-3 w-3" /> Egreso
                          </span>
                        )}
                      </td>
                      <td className="px-6 py-4 font-medium text-foreground">{mov.description}</td>
                      <td className={`px-6 py-4 font-bold ${mov.movement_type === 'IN' ? 'text-emerald-600' : 'text-red-600'}`}>
                        {mov.movement_type === 'IN' ? '+' : '-'}{fmt(mov.amount)}
                      </td>
                      <td className="px-6 py-4 text-right">
                        <Button size="sm" variant="ghost" className="h-8 w-8 p-0" title="Descargar Recibo">
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

      {/* Modal Nuevo Movimiento */}
      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Registrar Movimiento Manual">
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Tipo de Movimiento *</label>
            <div className="grid grid-cols-2 gap-3">
              <label className={`border rounded-md p-3 flex items-center justify-center gap-2 cursor-pointer transition-colors ${form.movement_type === 'IN' ? 'bg-emerald-50 border-emerald-500 text-emerald-700 dark:bg-emerald-900/20' : 'bg-background text-muted-foreground hover:bg-muted'}`}>
                <input type="radio" name="movement_type" value="IN" className="sr-only" checked={form.movement_type === 'IN'} onChange={(e) => setForm({...form, movement_type: e.target.value})} />
                <ArrowUpRight className="h-4 w-4" /> Ingreso
              </label>
              <label className={`border rounded-md p-3 flex items-center justify-center gap-2 cursor-pointer transition-colors ${form.movement_type === 'OUT' ? 'bg-red-50 border-red-500 text-red-700 dark:bg-red-900/20' : 'bg-background text-muted-foreground hover:bg-muted'}`}>
                <input type="radio" name="movement_type" value="OUT" className="sr-only" checked={form.movement_type === 'OUT'} onChange={(e) => setForm({...form, movement_type: e.target.value})} />
                <ArrowDownRight className="h-4 w-4" /> Egreso
              </label>
            </div>
          </div>
          
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Monto *</label>
            <div className="relative">
              <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">$</span>
              <Input type="number" name="amount" value={form.amount} onChange={e => setForm({...form, amount: e.target.value})} className="pl-7" min="0.01" step="0.01" required />
            </div>
          </div>

          <div className="space-y-1.5">
            <label className="text-sm font-medium">Descripción / Concepto *</label>
            <Input type="text" name="description" value={form.description} onChange={e => setForm({...form, description: e.target.value})} placeholder="Ej. Pago de expensas, compra de resmas..." required />
          </div>

          <div className="pt-2 flex justify-end gap-3 border-t border-border mt-4 pt-4">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>Cancelar</Button>
            <Button type="submit" disabled={saving}>
              {saving ? 'Guardando...' : 'Guardar Movimiento'}
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
