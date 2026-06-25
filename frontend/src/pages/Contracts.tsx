import { useState, useEffect } from 'react'
import { Plus, Search, Download, CheckCircle, XCircle, FileText, Calendar, DollarSign, X } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Modal } from '@/components/ui/modal'
import { fetchApi, API_URL } from '@/services/api'
import { propertiesService } from '@/services/properties'

// ─── Tipos alineados con el backend ───────────────────────────────────────────
interface Contract {
  id: string
  tenant_id: string
  property_id: string
  start_date: string
  end_date: string
  original_rent_amount: string | number
  current_rent_amount: string | number | null
  adjustment_method: string | null
  adjustment_frequency: string | null
  automation_mode: string | null
  fixed_percentage: string | number | null
  status: string | null
}

interface CreateContractForm {
  property_id: string
  start_date: string
  end_date: string
  original_rent_amount: string
  adjustment_method: string
  adjustment_frequency: string
  automation_mode: string
  fixed_percentage: string
  notification_days_before: string
}

const ADJUSTMENT_METHODS = [
  { value: 'MANUAL', label: 'Manual' },
  { value: 'FIXED_PERCENTAGE', label: 'Porcentaje Fijo' },
  { value: 'IPC', label: 'IPC (Inflación)' },
  { value: 'ICL', label: 'ICL (Índice Casa Propia)' },
  { value: 'CASA_PROPIA', label: 'Casa Propia' },
  { value: 'CUSTOM', label: 'Personalizado' },
]

const ADJUSTMENT_FREQUENCIES = [
  { value: 'MONTHLY', label: 'Mensual' },
  { value: 'BIMONTHLY', label: 'Bimestral' },
  { value: 'QUARTERLY', label: 'Trimestral' },
  { value: 'SEMESTER', label: 'Semestral' },
  { value: 'ANNUAL', label: 'Anual' },
]

const AUTOMATION_MODES = [
  { value: 'MANUAL', label: 'Manual (aprobación requerida)' },
  { value: 'SEMIAUTOMATIC', label: 'Semiautomático (notifica y aplica)' },
  { value: 'AUTOMATIC', label: 'Automático (sin intervención)' },
]

const EMPTY_FORM: CreateContractForm = {
  property_id: '',
  start_date: '',
  end_date: '',
  original_rent_amount: '',
  adjustment_method: 'IPC',
  adjustment_frequency: 'QUARTERLY',
  automation_mode: 'SEMIAUTOMATIC',
  fixed_percentage: '',
  notification_days_before: '30',
}

export default function Contracts() {
  const [activeTab, setActiveTab] = useState<'contracts' | 'pending'>('contracts')
  const [contracts, setContracts] = useState<Contract[]>([])
  const [pendingAdjustments, setPendingAdjustments] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [toast, setToast] = useState<{ msg: string; type: 'success' | 'error' } | null>(null)

  // Modal nuevo contrato
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [form, setForm] = useState<CreateContractForm>(EMPTY_FORM)
  const [properties, setProperties] = useState<any[]>([])
  const [saving, setSaving] = useState(false)
  const [formError, setFormError] = useState('')

  useEffect(() => {
    if (activeTab === 'contracts') loadContracts()
    else loadPendingAdjustments()
  }, [activeTab])

  const showToast = (msg: string, type: 'success' | 'error' = 'success') => {
    setToast({ msg, type })
    setTimeout(() => setToast(null), 3500)
  }

  const loadContracts = async () => {
    try {
      setLoading(true)
      const data = await fetchApi('/contracts')
      setContracts(Array.isArray(data) ? data : data?.items || data?.data || [])
    } catch (err: any) {
      console.error(err)
      showToast('Error al cargar contratos', 'error')
      setContracts([])
    } finally {
      setLoading(false)
    }
  }

  const loadPendingAdjustments = async () => {
    try {
      setLoading(true)
      const data = await fetchApi('/contracts/adjustments/pending')
      setPendingAdjustments(data?.items || data?.data || (Array.isArray(data) ? data : []))
    } catch (err) {
      console.error(err)
      setPendingAdjustments([])
    } finally {
      setLoading(false)
    }
  }

  const openNewContractModal = async () => {
    setForm(EMPTY_FORM)
    setFormError('')
    setIsModalOpen(true)
    // Cargar propiedades disponibles para el select
    try {
      const data = await propertiesService.getAll(100, 0)
      const list = Array.isArray(data) ? data : data?.data || []
      // Solo propiedades disponibles
      setProperties(Array.isArray(list) ? list.filter((p: any) => p.status !== 'Alquilada' && p.status !== 'Vendida') : [])
    } catch {
      setProperties([])
    }
  }

  const handleFormChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target
    setForm(prev => ({ ...prev, [name]: value }))
  }

  const handleDownloadPdf = async (contractId: string) => {
    try {
      const token = localStorage.getItem('token');
      const response = await fetch(`${API_URL}/api/contracts/${contractId}/pdf`, {
        headers: {
          ...(token ? { Authorization: `Bearer ${token}` } : {})
        }
      });
      
      if (!response.ok) {
        throw new Error('No se pudo descargar el PDF');
      }
      
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `contrato_${contractId}.pdf`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      a.remove();
    } catch (err) {
      console.error(err);
      showToast('Error al descargar el PDF', 'error');
    }
  }

  const handleSubmitContract = async (e: React.FormEvent) => {
    e.preventDefault()
    setFormError('')

    if (!form.property_id) return setFormError('Selecciona una propiedad.')
    if (!form.start_date || !form.end_date) return setFormError('Las fechas de inicio y fin son obligatorias.')
    if (!form.original_rent_amount || isNaN(Number(form.original_rent_amount))) return setFormError('Ingresa un monto de alquiler válido.')
    if (new Date(form.end_date) <= new Date(form.start_date)) return setFormError('La fecha de fin debe ser posterior a la de inicio.')

    setSaving(true)
    try {
      const payload: any = {
        property_id: form.property_id,
        start_date: form.start_date,
        end_date: form.end_date,
        original_rent_amount: parseFloat(form.original_rent_amount),
        adjustment_method: form.adjustment_method || null,
        adjustment_frequency: form.adjustment_frequency || null,
        automation_mode: form.automation_mode || null,
        notification_days_before: form.notification_days_before ? parseInt(form.notification_days_before) : null,
      }

      if (form.adjustment_method === 'FIXED_PERCENTAGE' && form.fixed_percentage) {
        payload.fixed_percentage = parseFloat(form.fixed_percentage)
      }

      await fetchApi('/contracts', {
        method: 'POST',
        body: JSON.stringify(payload),
      })

      setIsModalOpen(false)
      showToast('Contrato creado correctamente', 'success')
      loadContracts()
    } catch (err: any) {
      setFormError(err.message || 'Error al crear el contrato')
    } finally {
      setSaving(false)
    }
  }

  const handleApprove = async (id: string) => {
    try {
      await fetchApi(`/contracts/adjustments/${id}/approve`, {
        method: 'POST',
        body: JSON.stringify({ new_amount: null, notes: 'Aprobado manualmente' }),
      })
      showToast('Ajuste aprobado correctamente.', 'success')
      loadPendingAdjustments()
    } catch (err) {
      showToast('Error al aprobar el ajuste.', 'error')
    }
  }

  const handleReject = async (id: string) => {
    const reason = window.prompt('Motivo de rechazo:')
    if (reason === null) return
    try {
      await fetchApi(`/contracts/adjustments/${id}/reject`, {
        method: 'POST',
        body: JSON.stringify({ reason }),
      })
      showToast('Ajuste rechazado.', 'success')
      loadPendingAdjustments()
    } catch {
      showToast('Error al rechazar el ajuste.', 'error')
    }
  }

  const formatMonto = (val: string | number | null | undefined) => {
    if (val == null) return '-'
    return `$${Number(val).toLocaleString('es-AR')}`
  }

  return (
    <div className="space-y-6 relative">
      {/* Toast */}
      {toast && (
        <div className={`fixed bottom-4 right-4 flex items-center gap-2 px-4 py-3 rounded-lg shadow-xl z-50 text-white text-sm font-medium transition-all ${toast.type === 'error' ? 'bg-red-600' : 'bg-emerald-600'}`}>
          {toast.type === 'success' ? <CheckCircle className="h-4 w-4" /> : <XCircle className="h-4 w-4" />}
          {toast.msg}
        </div>
      )}

      {/* Encabezado */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-foreground">Contratos</h1>
          <p className="text-muted-foreground">Gestión de contratos de alquiler y vencimientos.</p>
        </div>
        <Button className="gap-2" onClick={openNewContractModal}>
          <Plus className="h-4 w-4" /> Nuevo Contrato
        </Button>
      </div>

      {/* Tabs */}
      <div className="flex space-x-4 border-b border-border">
        {(['contracts', 'pending'] as const).map((tab) => (
          <button
            key={tab}
            className={`pb-2 px-1 text-sm font-medium transition-colors border-b-2 ${
              activeTab === tab
                ? 'border-primary text-primary'
                : 'border-transparent text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setActiveTab(tab)}
          >
            {tab === 'contracts' ? 'Listado de Contratos' : 'Ajustes Pendientes'}
          </button>
        ))}
      </div>

      {/* ── Tab: Listado de Contratos ──────────────────────────────────────── */}
      {activeTab === 'contracts' && (
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Listado de Contratos</CardTitle>
              <div className="relative w-64">
                <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                <Input placeholder="Buscar contrato..." className="pl-8" />
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                  <tr>
                    <th className="px-6 py-3 font-medium">Propiedad</th>
                    <th className="px-6 py-3 font-medium">Inicio</th>
                    <th className="px-6 py-3 font-medium">Fin</th>
                    <th className="px-6 py-3 font-medium">Monto Actual</th>
                    <th className="px-6 py-3 font-medium">Ajuste</th>
                    <th className="px-6 py-3 font-medium">Estado</th>
                    <th className="px-6 py-3 font-medium">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {loading ? (
                    <tr><td colSpan={7} className="px-6 py-8 text-center text-muted-foreground">Cargando...</td></tr>
                  ) : contracts.length === 0 ? (
                    <tr>
                      <td colSpan={7} className="px-6 py-12 text-center">
                        <div className="flex flex-col items-center gap-3 text-muted-foreground">
                          <FileText className="h-10 w-10 opacity-30" />
                          <p>No hay contratos registrados.</p>
                          <Button variant="outline" size="sm" onClick={openNewContractModal} className="gap-2">
                            <Plus className="h-4 w-4" /> Crear primer contrato
                          </Button>
                        </div>
                      </td>
                    </tr>
                  ) : (
                    contracts.map((c) => (
                      <tr key={c.id} className="border-b last:border-0 hover:bg-muted/50 transition-colors">
                        <td className="px-6 py-4 font-medium text-foreground">{c.property_id}</td>
                        <td className="px-6 py-4 text-muted-foreground">{c.start_date}</td>
                        <td className="px-6 py-4 text-muted-foreground">{c.end_date}</td>
                        <td className="px-6 py-4 font-semibold">{formatMonto(c.current_rent_amount ?? c.original_rent_amount)}</td>
                        <td className="px-6 py-4">
                          <span className="text-xs px-2 py-1 rounded-full bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300">
                            {c.adjustment_method?.replace('_', ' ') || 'Manual'}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          <span className="px-2.5 py-1 rounded-full text-xs font-medium bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300">
                            {c.status || 'Activo'}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          <Button
                            variant="ghost"
                            size="sm"
                            className="gap-2 text-muted-foreground hover:text-foreground"
                            onClick={() => handleDownloadPdf(c.id)}
                          >
                            <Download className="h-4 w-4" /> PDF
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
      )}

      {/* ── Tab: Ajustes Pendientes ────────────────────────────────────────── */}
      {activeTab === 'pending' && (
        <Card>
          <CardHeader><CardTitle>Ajustes Pendientes de Aprobación</CardTitle></CardHeader>
          <CardContent>
            <div className="rounded-md border overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                  <tr>
                    <th className="px-6 py-3 font-medium">Contrato</th>
                    <th className="px-6 py-3 font-medium">Inquilino</th>
                    <th className="px-6 py-3 font-medium">Monto Actual</th>
                    <th className="px-6 py-3 font-medium">% Aplicado</th>
                    <th className="px-6 py-3 font-medium">Nuevo Monto</th>
                    <th className="px-6 py-3 font-medium">Fecha Efectiva</th>
                    <th className="px-6 py-3 font-medium">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {loading ? (
                    <tr><td colSpan={7} className="px-6 py-8 text-center text-muted-foreground">Cargando...</td></tr>
                  ) : pendingAdjustments.length === 0 ? (
                    <tr><td colSpan={7} className="px-6 py-8 text-center text-muted-foreground">No hay ajustes pendientes.</td></tr>
                  ) : (
                    pendingAdjustments.map((adj) => (
                      <tr key={adj.adjustment_id} className="border-b last:border-0 hover:bg-muted/50">
                        <td className="px-6 py-4">{adj.contract_number}</td>
                        <td className="px-6 py-4">{adj.tenant_name}</td>
                        <td className="px-6 py-4">{formatMonto(adj.current_rent)}</td>
                        <td className="px-6 py-4">{adj.adjustment_percent}%</td>
                        <td className="px-6 py-4 font-semibold text-primary">{formatMonto(adj.new_rent)}</td>
                        <td className="px-6 py-4">{adj.effective_date}</td>
                        <td className="px-6 py-4 flex gap-2">
                          <Button variant="outline" size="sm" className="text-green-600 border-green-200 hover:bg-green-50" onClick={() => handleApprove(adj.adjustment_id)}>
                            <CheckCircle className="h-4 w-4 mr-1" /> Aprobar
                          </Button>
                          <Button variant="outline" size="sm" className="text-red-600 border-red-200 hover:bg-red-50" onClick={() => handleReject(adj.adjustment_id)}>
                            <XCircle className="h-4 w-4 mr-1" /> Rechazar
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
      )}

      {/* ── Modal: Nuevo Contrato ────────────────────────────────────────────── */}
      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Nuevo Contrato de Alquiler">
        <form onSubmit={handleSubmitContract} className="space-y-5">

          {formError && (
            <div className="p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 border border-red-200 dark:border-red-800 rounded-md">
              {formError}
            </div>
          )}

          {/* Propiedad */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
              <FileText className="h-4 w-4 text-muted-foreground" /> Propiedad *
            </label>
            <select
              name="property_id"
              value={form.property_id}
              onChange={handleFormChange}
              required
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            >
              <option value="">— Seleccionar propiedad —</option>
              {properties.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.title} {p.location ? `· ${p.location}` : ''}
                </option>
              ))}
              {properties.length === 0 && <option disabled>No hay propiedades disponibles</option>}
            </select>
          </div>

          {/* Fechas */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
                <Calendar className="h-4 w-4 text-muted-foreground" /> Inicio *
              </label>
              <Input type="date" name="start_date" value={form.start_date} onChange={handleFormChange} required />
            </div>
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
                <Calendar className="h-4 w-4 text-muted-foreground" /> Vencimiento *
              </label>
              <Input type="date" name="end_date" value={form.end_date} onChange={handleFormChange} required />
            </div>
          </div>

          {/* Monto */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground flex items-center gap-1.5">
              <DollarSign className="h-4 w-4 text-muted-foreground" /> Monto mensual de alquiler *
            </label>
            <div className="relative">
              <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground text-sm">$</span>
              <Input
                type="number"
                name="original_rent_amount"
                value={form.original_rent_amount}
                onChange={handleFormChange}
                placeholder="150000"
                className="pl-7"
                min="0"
                step="0.01"
                required
              />
            </div>
          </div>

          {/* Método de ajuste */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground">Método de ajuste</label>
            <select
              name="adjustment_method"
              value={form.adjustment_method}
              onChange={handleFormChange}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            >
              {ADJUSTMENT_METHODS.map((m) => (
                <option key={m.value} value={m.value}>{m.label}</option>
              ))}
            </select>
          </div>

          {/* Porcentaje fijo (solo si método = FIXED_PERCENTAGE) */}
          {form.adjustment_method === 'FIXED_PERCENTAGE' && (
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground">Porcentaje fijo (%)</label>
              <Input
                type="number"
                name="fixed_percentage"
                value={form.fixed_percentage}
                onChange={handleFormChange}
                placeholder="Ej: 10"
                min="0"
                max="100"
                step="0.01"
              />
            </div>
          )}

          {/* Frecuencia */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground">Frecuencia de ajuste</label>
              <select
                name="adjustment_frequency"
                value={form.adjustment_frequency}
                onChange={handleFormChange}
                className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
              >
                {ADJUSTMENT_FREQUENCIES.map((f) => (
                  <option key={f.value} value={f.value}>{f.label}</option>
                ))}
              </select>
            </div>
            <div className="space-y-1.5">
              <label className="text-sm font-medium text-foreground">Días de aviso</label>
              <Input
                type="number"
                name="notification_days_before"
                value={form.notification_days_before}
                onChange={handleFormChange}
                placeholder="30"
                min="1"
                max="365"
              />
            </div>
          </div>

          {/* Modo de automatización */}
          <div className="space-y-1.5">
            <label className="text-sm font-medium text-foreground">Modo de automatización</label>
            <select
              name="automation_mode"
              value={form.automation_mode}
              onChange={handleFormChange}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            >
              {AUTOMATION_MODES.map((m) => (
                <option key={m.value} value={m.value}>{m.label}</option>
              ))}
            </select>
            <p className="text-xs text-muted-foreground mt-1">
              {form.automation_mode === 'MANUAL' && 'Los ajustes se propondrán y deberás aprobarlos manualmente.'}
              {form.automation_mode === 'SEMIAUTOMATIC' && 'Se notificará antes de aplicar el ajuste. Podrás modificarlo.'}
              {form.automation_mode === 'AUTOMATIC' && 'Los ajustes se aplicarán automáticamente en la fecha programada.'}
            </p>
          </div>

          {/* Acciones */}
          <div className="pt-2 flex justify-end gap-3 border-t border-border">
            <Button type="button" variant="outline" onClick={() => setIsModalOpen(false)} disabled={saving}>
              <X className="h-4 w-4 mr-1" /> Cancelar
            </Button>
            <Button type="submit" disabled={saving} className="gap-2">
              {saving ? (
                <><div className="h-4 w-4 rounded-full border-2 border-white border-t-transparent animate-spin" /> Guardando...</>
              ) : (
                <><Plus className="h-4 w-4" /> Crear Contrato</>
              )}
            </Button>
          </div>
        </form>
      </Modal>
    </div>
  )
}
