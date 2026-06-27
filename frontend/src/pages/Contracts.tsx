import { useState, useEffect } from 'react'
import { Plus, Search, Download, CheckCircle, XCircle, FileText } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { fetchApi, API_URL } from '@/services/api'
import ContractWizard from '@/components/contracts/ContractWizard'

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


export default function Contracts() {
  const [activeTab, setActiveTab] = useState<'contracts' | 'pending'>('contracts')
  const [contracts, setContracts] = useState<Contract[]>([])
  const [pendingAdjustments, setPendingAdjustments] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [toast, setToast] = useState<{ msg: string; type: 'success' | 'error' } | null>(null)

  // Modal nuevo contrato
  const [isModalOpen, setIsModalOpen] = useState(false)

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

  const openNewContractModal = () => {
    setIsModalOpen(true)
  }
  const handleDownloadPdf = async (contractId: string) => {
    try {
      const token = localStorage.getItem('token');
      
      // Intentar V2 (PDF nativo)
      let response = await fetch(`${API_URL}/api/contracts/v2/${contractId}/pdf`, {
        headers: { ...(token ? { Authorization: `Bearer ${token}` } : {}) }
      });

      if (response.ok) {
        // Es un PDF real generado por GenPDF en Fase 3
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `contrato_${contractId}.pdf`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
        return;
      }

      // Fallback a V1 (HTML string patch)
      response = await fetch(`${API_URL}/api/contracts/${contractId}/pdf`, {
        headers: { ...(token ? { Authorization: `Bearer ${token}` } : {}) }
      });
      
      if (!response.ok) throw new Error('No se pudo generar el documento');
      
      const htmlText = await response.text();
      const newWin = window.open('', '_blank');
      if (newWin) {
        newWin.document.open();
        newWin.document.write(htmlText);
        newWin.document.close();
        setTimeout(() => newWin.print(), 250);
      } else {
        showToast('Por favor permite pop-ups para ver el documento', 'error');
      }
    } catch (err) {
      console.error(err);
      showToast('Error al obtener el documento', 'error');
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

      {/* ── Modal: Nuevo Contrato V2 ────────────────────────────────────────────── */}
      {isModalOpen && (
        <ContractWizard
          onClose={() => setIsModalOpen(false)}
          onSuccess={() => {
            setIsModalOpen(false)
            showToast('Contrato creado exitosamente')
            loadContracts()
          }}
        />
      )}
    </div>
  )
}
