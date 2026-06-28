import { useState, useEffect } from 'react'
import { DollarSign, Search, CheckCircle, CreditCard, Plus, FileText, AlertCircle, Calendar } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Modal } from '@/components/ui/modal'
import { fetchApi } from '@/services/api'

interface Invoice {
  id: string
  contract_id: string | null
  amount: string | number
  commission: string | number | null
  status: string | null
  due_date: string
}

interface Contract {
  id: string
  property_id: string
  original_rent_amount: string | number
  current_rent_amount: string | number | null
  status: string | null
}

interface CreateInvoiceForm {
  contract_id: string
  amount: string
  commission: string
  due_date: string
}

const EMPTY_FORM: CreateInvoiceForm = {
  contract_id: '',
  amount: '',
  commission: '',
  due_date: '',
}

export function BillingTab() {
  const [invoices, setInvoices] = useState<Invoice[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [searchTerm, setSearchTerm] = useState('')

  // Modal
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [form, setForm] = useState<CreateInvoiceForm>(EMPTY_FORM)
  const [contracts, setContracts] = useState<Contract[]>([])
  const [saving, setSaving] = useState(false)
  const [formError, setFormError] = useState('')

  useEffect(() => {
    loadInvoices()
  }, [])

  const loadInvoices = async () => {
    try {
      setLoading(true)
      setError('')
      const data = await fetchApi('/financials/invoices')
      setInvoices(Array.isArray(data) ? data : data?.items || data?.data || [])
    } catch (err: any) {
      setError(err.message || 'Error al cargar cobros')
      setInvoices([])
    } finally {
      setLoading(false)
    }
  }

  const markAsPaid = async (id: string) => {
    try {
      await fetchApi(`/financials/invoices/${id}/pay_manual`, { method: 'POST' })
      loadInvoices()
    } catch (err: any) {
      setError(err.message || 'Error al registrar pago')
    }
  }

  const payWithMP = async (id: string) => {
    try {
      const data = await fetchApi(`/payments/checkout/rent/${id}`, { method: 'POST' })
      if (data?.init_point) {
        window.location.href = data.init_point
      }
    } catch (err: any) {
       setError(err.message || 'Error al generar pago con Mercado Pago')
    }
  }

  const openNewInvoiceModal = async () => {
    setForm(EMPTY_FORM)
    setFormError('')
    setIsModalOpen(true)
    try {
      const data = await fetchApi('/contracts')
      const list: Contract[] = Array.isArray(data) ? data : data?.data || []
      setContracts(Array.isArray(list) ? list.filter(c => !c.status || c.status === 'Activo' || c.status === 'active') : [])
    } catch {
      setContracts([])
    }
  }

  const handleContractChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const contractId = e.target.value
    const selected = contracts.find(c => c.id === contractId)
    setForm(prev => ({
      ...prev,
      contract_id: contractId,
      amount: selected
        ? String(selected.current_rent_amount ?? selected.original_rent_amount ?? '')
        : prev.amount,
    }))
  }

  const handleFormChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setForm(prev => ({ ...prev, [name]: value }))
  }

  const handleSubmitInvoice = async (e: React.FormEvent) => {
    e.preventDefault()
    setFormError('')
    setSaving(true)
    try {
      await fetchApi('/financials/invoices', {
        method: 'POST',
        body: JSON.stringify({
          contract_id: form.contract_id,
          amount: parseFloat(form.amount),
          commission: form.commission ? parseFloat(form.commission) : 0,
          due_date: form.due_date,
        }),
      })
      setIsModalOpen(false)
      loadInvoices()
    } catch (err: any) {
      setFormError(err.message || 'Error al generar el cobro')
    } finally {
      setSaving(false)
    }
  }

  const fmt = (val: string | number | null | undefined) => {
    if (val == null) return '-'
    return `$${Number(val).toLocaleString('es-AR')}`
  }

  const filteredInvoices = invoices.filter(i => 
    i.contract_id?.toLowerCase().includes(searchTerm.toLowerCase()) ||
    i.status?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  return (
    <div className="space-y-4 animate-in fade-in slide-in-from-bottom-4 duration-500">
      {error && (
        <div className="p-4 text-red-600 bg-red-50 dark:bg-red-900/20 rounded-md text-sm flex items-center gap-2 border border-red-200">
           <AlertCircle className="h-4 w-4" /> {error}
        </div>
      )}

      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-2 border-b border-border mb-4">
          <CardTitle>Cuentas Corrientes y Cuotas</CardTitle>
          <div className="flex items-center gap-2">
            <div className="relative w-64">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input placeholder="Buscar cuota..." className="pl-8" value={searchTerm} onChange={e => setSearchTerm(e.target.value)} />
            </div>
            <Button className="gap-2" onClick={openNewInvoiceModal}>
              <Plus className="h-4 w-4" /> Nueva Cuota
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50 border-b border-border">
                <tr>
                  <th className="px-6 py-3 font-medium">Contrato</th>
                  <th className="px-6 py-3 font-medium">Monto</th>
                  <th className="px-6 py-3 font-medium">Comisión</th>
                  <th className="px-6 py-3 font-medium">Vencimiento</th>
                  <th className="px-6 py-3 font-medium">Estado</th>
                  <th className="px-6 py-3 font-medium">Acciones</th>
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr><td colSpan={6} className="px-6 py-8 text-center text-muted-foreground">Cargando...</td></tr>
                ) : filteredInvoices.length === 0 ? (
                  <tr><td colSpan={6} className="px-6 py-12 text-center text-muted-foreground flex flex-col items-center gap-2">
                    <DollarSign className="h-8 w-8 opacity-20" /> No hay cuotas registradas.
                  </td></tr>
                ) : (
                  filteredInvoices.map((inv) => (
                    <tr key={inv.id} className="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4 font-medium text-foreground">{inv.contract_id?.slice(0,8) || '-'}</td>
                      <td className="px-6 py-4 font-semibold">{fmt(inv.amount)}</td>
                      <td className="px-6 py-4 text-muted-foreground">{fmt(inv.commission)}</td>
                      <td className="px-6 py-4 text-muted-foreground">{inv.due_date}</td>
                      <td className="px-6 py-4">
                        <span className={`px-2.5 py-1 rounded-full text-xs font-medium ${
                          inv.status === 'PAID'
                            ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300 border border-green-200'
                            : inv.status === 'OVERDUE'
                            ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300 border border-red-200'
                            : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300 border border-yellow-200'
                        }`}>
                          {inv.status === 'PAID' ? 'Pagado' : inv.status === 'OVERDUE' ? 'Vencido' : 'Pendiente'}
                        </span>
                      </td>
                      <td className="px-6 py-4">
                        {inv.status !== 'PAID' && (
                          <div className="flex gap-2">
                            <Button size="sm" variant="outline" className="gap-1.5 h-8 text-xs" onClick={() => markAsPaid(inv.id)}>
                              <CheckCircle className="h-3.5 w-3.5" /> Manual
                            </Button>
                            <Button size="sm" className="gap-1.5 bg-blue-600 hover:bg-blue-700 h-8 text-xs text-white" onClick={() => payWithMP(inv.id)}>
                              <CreditCard className="h-3.5 w-3.5" /> MP
                            </Button>
                          </div>
                        )}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Generar Cuota Manual">
        <form onSubmit={handleSubmitInvoice} className="space-y-4">
          {formError && <div className="p-3 text-sm text-red-600 bg-red-50 rounded-md border border-red-200">{formError}</div>}
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Contrato *</label>
            <select name="contract_id" value={form.contract_id} onChange={handleContractChange} required className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary focus:outline-none">
              <option value="">— Seleccionar contrato —</option>
              {contracts.map(c => (
                <option key={c.id} value={c.id}>
                  {c.id.slice(0, 8)}... · Alquiler: ${Number(c.current_rent_amount ?? c.original_rent_amount).toLocaleString('es-AR')}
                </option>
              ))}
            </select>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1.5">
              <label className="text-sm font-medium">Monto *</label>
              <div className="relative">
                <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">$</span>
                <Input type="number" name="amount" value={form.amount} onChange={handleFormChange} className="pl-7" min="0" step="0.01" required />
              </div>
            </div>
            <div className="space-y-1.5">
              <label className="text-sm font-medium">Comisión</label>
              <div className="relative">
                <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">$</span>
                <Input type="number" name="commission" value={form.commission} onChange={handleFormChange} className="pl-7" min="0" step="0.01" />
              </div>
            </div>
          </div>
          <div className="space-y-1.5">
            <label className="text-sm font-medium">Vencimiento *</label>
            <Input type="date" name="due_date" value={form.due_date} onChange={handleFormChange} required />
          </div>
          <div className="pt-2 flex justify-end gap-3 border-t border-border mt-4 pt-4">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)}>Cancelar</Button>
            <Button type="submit" disabled={saving}>
              {saving ? 'Generando...' : 'Generar Cuota'}
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
