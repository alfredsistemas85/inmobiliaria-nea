import { useState, useEffect } from 'react'
import { DollarSign, Search, CheckCircle, CreditCard, Plus, X, FileText, Calendar } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Modal } from '@/components/ui/modal'
import { fetchApi } from '@/services/api'

// ─── Tipos alineados con el backend ───────────────────────────────────────────
interface Invoice {
  id: string
  contract_id: string | null
  amount: string | number
  commission: string | number | null
  status: string | null
  due_date: string
}

interface Liquidation {
  owner_name: string
  property_title: string
  total_collected: string | number
  commission_deducted: string | number
  net_to_transfer: string | number
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

export default function Financials() {
  const [activeTab, setActiveTab] = useState<'invoices' | 'liquidations'>('invoices')
  const [invoices, setInvoices] = useState<Invoice[]>([])
  const [liquidations, setLiquidations] = useState<Liquidation[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  // Modal Cobro Mensual
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [form, setForm] = useState<CreateInvoiceForm>(EMPTY_FORM)
  const [contracts, setContracts] = useState<Contract[]>([])
  const [saving, setSaving] = useState(false)
  const [formError, setFormError] = useState('')
  const [toast, setToast] = useState<{ msg: string; type: 'success' | 'error' } | null>(null)

  useEffect(() => {
    if (activeTab === 'invoices') loadInvoices()
    else loadLiquidations()
  }, [activeTab])

  const showToast = (msg: string, type: 'success' | 'error' = 'success') => {
    setToast({ msg, type })
    setTimeout(() => setToast(null), 3500)
  }

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

  const loadLiquidations = async () => {
    try {
      setLoading(true)
      setError('')
      const data = await fetchApi('/financials/liquidations')
      setLiquidations(Array.isArray(data) ? data : data?.items || data?.data || [])
    } catch (err: any) {
      setError(err.message || 'Error al cargar liquidaciones')
      setLiquidations([])
    } finally {
      setLoading(false)
    }
  }

  const markAsPaid = async (id: string) => {
    try {
      await fetchApi(`/financials/invoices/${id}/pay_manual`, { method: 'POST' })
      showToast('Marcado como pagado correctamente', 'success')
      loadInvoices()
    } catch {
      showToast('Error al marcar como pagado', 'error')
    }
  }

  const payWithMP = async (id: string) => {
    try {
      const data = await fetchApi(`/payments/checkout/rent/${id}`, { method: 'POST' })
      if (data?.init_point) {
        window.location.href = data.init_point
      } else {
        showToast('No se pudo generar el link de pago', 'error')
      }
    } catch {
      showToast('Error al generar pago con Mercado Pago', 'error')
    }
  }

  // ── Abrir modal y cargar contratos activos ─────────────────────────────────
  const openNewInvoiceModal = async () => {
    setForm(EMPTY_FORM)
    setFormError('')
    setIsModalOpen(true)

    // Cargar contratos para el select
    try {
      const data = await fetchApi('/contracts')
      const list: Contract[] = Array.isArray(data) ? data : data?.data || []
      setContracts(Array.isArray(list) ? list.filter(c => !c.status || c.status === 'Activo' || c.status === 'active') : [])
    } catch {
      setContracts([])
    }
  }

  // Al seleccionar un contrato, prellenar el monto con el alquiler actual
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

    if (!form.contract_id) return setFormError('Seleccioná un contrato.')
    if (!form.amount || isNaN(Number(form.amount)) || Number(form.amount) <= 0)
      return setFormError('El monto debe ser un número positivo.')
    if (!form.due_date) return setFormError('La fecha de vencimiento es obligatoria.')

    setSaving(true)
    try {
      const payload = {
        contract_id: form.contract_id,
        amount: parseFloat(form.amount),
        commission: form.commission ? parseFloat(form.commission) : 0,
        due_date: form.due_date,
      }

      await fetchApi('/financials/invoices', {
        method: 'POST',
        body: JSON.stringify(payload),
      })

      setIsModalOpen(false)
      showToast('Cobro generado correctamente', 'success')
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

  return (
    <div className="space-y-6">
      {/* Toast */}
      {toast && (
        <div className={`fixed bottom-4 right-4 flex items-center gap-2 px-4 py-3 rounded-lg shadow-xl z-50 text-white text-sm font-medium ${toast.type === 'error' ? 'bg-red-600' : 'bg-emerald-600'}`}>
          {toast.type === 'success' ? <CheckCircle className="h-4 w-4" /> : <X className="h-4 w-4" />}
          {toast.msg}
        </div>
      )}

      {/* Encabezado */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Finanzas</h1>
          <p className="text-muted-foreground">Cobro de alquileres y liquidaciones a propietarios.</p>
        </div>
        <Button className="gap-2" onClick={openNewInvoiceModal}>
          <DollarSign className="h-4 w-4" /> Generar Cobro Mensual
        </Button>
      </div>

      {/* Tabs */}
      <div className="flex gap-4 border-b border-border">
        {([
          { key: 'invoices', label: 'Alquileres / Expensas' },
          { key: 'liquidations', label: 'Liquidaciones a Propietarios' },
        ] as const).map(({ key, label }) => (
          <button
            key={key}
            className={`pb-2 px-1 font-medium text-sm transition-colors ${activeTab === key ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
            onClick={() => setActiveTab(key)}
          >
            {label}
          </button>
        ))}
      </div>

      {/* Error global */}
      {error && (
        <div className="p-4 text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 border border-red-100 dark:border-red-800 rounded-md text-sm">
          {error}
        </div>
      )}

      {/* ── Tabla ──────────────────────────────────────────────────────────── */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>{activeTab === 'invoices' ? 'Deudas y Cobros' : 'Liquidaciones Generadas'}</CardTitle>
            <div className="relative w-64">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input placeholder="Buscar..." className="pl-8" />
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                <tr>
                  {activeTab === 'invoices' ? (
                    <>
                      <th className="px-6 py-3 font-medium">Contrato</th>
                      <th className="px-6 py-3 font-medium">Monto</th>
                      <th className="px-6 py-3 font-medium">Comisión</th>
                      <th className="px-6 py-3 font-medium">Vencimiento</th>
                      <th className="px-6 py-3 font-medium">Estado</th>
                      <th className="px-6 py-3 font-medium">Acciones</th>
                    </>
                  ) : (
                    <>
                      <th className="px-6 py-3 font-medium">Propietario</th>
                      <th className="px-6 py-3 font-medium">Propiedad</th>
                      <th className="px-6 py-3 font-medium">Cobrado</th>
                      <th className="px-6 py-3 font-medium">Comisión Deducida</th>
                      <th className="px-6 py-3 font-medium">Neto a Transferir</th>
                    </>
                  )}
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr><td colSpan={6} className="px-6 py-8 text-center text-muted-foreground">Cargando...</td></tr>
                ) : activeTab === 'invoices' ? (
                  invoices.length === 0 ? (
                    <tr>
                      <td colSpan={6} className="px-6 py-12 text-center">
                        <div className="flex flex-col items-center gap-3 text-muted-foreground">
                          <DollarSign className="h-10 w-10 opacity-30" />
                          <p>No hay cobros registrados.</p>
                          <Button variant="outline" size="sm" onClick={openNewInvoiceModal} className="gap-2">
                            <Plus className="h-4 w-4" /> Generar primer cobro
                          </Button>
                        </div>
                      </td>
                    </tr>
                  ) : (
                    invoices.map((inv) => (
                      <tr key={inv.id} className="border-b last:border-0 hover:bg-muted/50 transition-colors">
                        <td className="px-6 py-4 font-medium text-foreground">{inv.contract_id || '-'}</td>
                        <td className="px-6 py-4 font-semibold">{fmt(inv.amount)}</td>
                        <td className="px-6 py-4 text-muted-foreground">{fmt(inv.commission)}</td>
                        <td className="px-6 py-4 text-muted-foreground">{inv.due_date}</td>
                        <td className="px-6 py-4">
                          <span className={`px-2.5 py-1 rounded-full text-xs font-medium ${
                            inv.status === 'PAID'
                              ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300'
                              : inv.status === 'OVERDUE'
                              ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300'
                              : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300'
                          }`}>
                            {inv.status === 'PAID' ? 'Pagado' : inv.status === 'OVERDUE' ? 'Vencido' : 'Pendiente'}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          {inv.status !== 'PAID' && (
                            <div className="flex gap-2">
                              <Button size="sm" variant="outline" className="gap-1.5" onClick={() => markAsPaid(inv.id)}>
                                <CheckCircle className="h-3.5 w-3.5" /> Manual
                              </Button>
                              <Button size="sm" className="gap-1.5 bg-blue-600 hover:bg-blue-700" onClick={() => payWithMP(inv.id)}>
                                <CreditCard className="h-3.5 w-3.5" /> MP
                              </Button>
                            </div>
                          )}
                        </td>
                      </tr>
                    ))
                  )
                ) : (
                  liquidations.length === 0 ? (
                    <tr><td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">No hay liquidaciones pendientes.</td></tr>
                  ) : (
                    liquidations.map((liq, idx) => (
                      <tr key={idx} className="border-b last:border-0 hover:bg-muted/50">
                        <td className="px-6 py-4 font-medium">{liq.owner_name}</td>
                        <td className="px-6 py-4">{liq.property_title}</td>
                        <td className="px-6 py-4 font-semibold">{fmt(liq.total_collected)}</td>
                        <td className="px-6 py-4 text-red-600 dark:text-red-400">-{fmt(liq.commission_deducted)}</td>
                        <td className="px-6 py-4 font-bold text-green-700 dark:text-green-400">{fmt(liq.net_to_transfer)}</td>
                      </tr>
                    ))
                  )
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      {/* ── Modal: Generar Cobro Mensual ──────────────────────────────────────── */}
      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Generar Cobro Mensual">
        <form onSubmit={handleSubmitInvoice} className="space-y-5">

          {formError && (
            <div className="p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 border border-red-200 dark:border-red-800 rounded-md">
              {formError}
            </div>
          )}

          {/* Contrato */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
              <FileText className="h-4 w-4 text-muted-foreground" /> Contrato *
            </label>
            <select
              name="contract_id"
              value={form.contract_id}
              onChange={handleContractChange}
              required
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            >
              <option value="">— Seleccionar contrato —</option>
              {contracts.map((c) => (
                <option key={c.id} value={c.id}>
                  {c.id.slice(0, 8)}... · Alquiler: ${Number(c.current_rent_amount ?? c.original_rent_amount).toLocaleString('es-AR')}
                </option>
              ))}
              {contracts.length === 0 && <option disabled>No hay contratos activos</option>}
            </select>
            <p className="text-xs text-muted-foreground">Al seleccionar un contrato el monto se precarga automáticamente.</p>
          </div>

          {/* Monto y comisión */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
                <DollarSign className="h-4 w-4 text-muted-foreground" /> Monto *
              </label>
              <div className="relative">
                <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground text-sm">$</span>
                <Input
                  type="number"
                  name="amount"
                  value={form.amount}
                  onChange={handleFormChange}
                  placeholder="150000"
                  className="pl-7"
                  min="0"
                  step="0.01"
                  required
                />
              </div>
            </div>
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
                <DollarSign className="h-4 w-4 text-muted-foreground" /> Comisión agencia
              </label>
              <div className="relative">
                <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground text-sm">$</span>
                <Input
                  type="number"
                  name="commission"
                  value={form.commission}
                  onChange={handleFormChange}
                  placeholder="0"
                  className="pl-7"
                  min="0"
                  step="0.01"
                />
              </div>
            </div>
          </div>

          {/* Vencimiento */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
              <Calendar className="h-4 w-4 text-muted-foreground" /> Fecha de vencimiento *
            </label>
            <Input
              type="date"
              name="due_date"
              value={form.due_date}
              onChange={handleFormChange}
              required
            />
          </div>

          {/* Resumen */}
          {form.amount && (
            <div className="bg-muted/50 rounded-lg p-4 space-y-2 border border-border">
              <p className="text-xs font-medium text-muted-foreground uppercase tracking-wide">Resumen</p>
              <div className="flex justify-between text-sm">
                <span className="text-muted-foreground">Monto alquiler</span>
                <span className="font-medium">${Number(form.amount || 0).toLocaleString('es-AR')}</span>
              </div>
              {form.commission && (
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Comisión agencia</span>
                  <span className="text-red-600">-${Number(form.commission || 0).toLocaleString('es-AR')}</span>
                </div>
              )}
              <div className="flex justify-between text-sm font-bold border-t border-border pt-2">
                <span>Neto al propietario</span>
                <span className="text-green-600">
                  ${(Number(form.amount || 0) - Number(form.commission || 0)).toLocaleString('es-AR')}
                </span>
              </div>
            </div>
          )}

          {/* Acciones */}
          <div className="pt-2 flex justify-end gap-3 border-t border-border">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)} disabled={saving}>
              <X className="h-4 w-4 mr-1" /> Cancelar
            </Button>
            <Button type="submit" disabled={saving} className="gap-2">
              {saving ? (
                <><div className="h-4 w-4 rounded-full border-2 border-white border-t-transparent animate-spin" /> Generando...</>
              ) : (
                <><DollarSign className="h-4 w-4" /> Generar Cobro</>
              )}
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
