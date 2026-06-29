import { useState, useEffect } from 'react'
import { Plus, Search, Download, CheckCircle, XCircle, FileText } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { fetchApi, API_URL } from '@/services/api'

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
  
  const [statusFilter, setStatusFilter] = useState<string>('Todos')
  const [searchQuery, setSearchQuery] = useState('')

  const navigate = useNavigate()

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
    navigate('/contracts/new')
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

  const handleRequestSignatures = async (contractId: string) => {
    try {
      setLoading(true)
      // 1. Fetch contract details to get participants
      const data = await fetchApi(`/contracts/v2/${contractId}`)
      if (!data || !data.participants || data.participants.length === 0) {
        showToast('El contrato no tiene participantes para firmar.', 'error')
        return
      }

      // 2. Build the request payload
      const payload = {
        requests: data.participants.map((p: any) => ({
          participant_id: p.id,
          signature_order: 1,
          required_signature: true,
          signature_type: "HANDDRAWN"
        }))
      }

      // 3. Send the signature requests
      const response = await fetchApi(`/signatures/contracts/${contractId}/signatures/request`, {
        method: 'POST',
        body: JSON.stringify(payload)
      })

      if (response && response.data) {
        const links = response.data.map((r: any) => r.link).join('\n');
        alert('Solicitud de firmas generada correctamente. Links para firmar (solo para pruebas):\n\n' + links);
      } else {
        showToast('Solicitud de firmas generada correctamente', 'success')
      }
      
      loadContracts()
    } catch (err: any) {
      console.error(err)
      showToast(err.message || 'Error al solicitar firmas', 'error')
    } finally {
      setLoading(false)
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
  
  const statusFiltersList = ['Todos', 'Draft', 'Pendientes Firma', 'Activos', 'Finalizados', 'Cancelados'];
  
  const filteredContracts = contracts.filter(c => {
    let matchesStatus = true;
    if (statusFilter !== 'Todos') {
       const mappedStatus = 
         statusFilter === 'Draft' ? 'DRAFT' :
         statusFilter === 'Pendientes Firma' ? 'PENDING_SIGNATURE' :
         statusFilter === 'Activos' ? 'ACTIVE' :
         statusFilter === 'Finalizados' ? 'FINISHED' :
         statusFilter === 'Cancelados' ? 'CANCELLED' : '';
       
       matchesStatus = c.status === mappedStatus || (c.status === null && statusFilter === 'Activos');
    }
    
    let matchesSearch = true;
    if (searchQuery) {
       const lowerQ = searchQuery.toLowerCase();
       // Simple search on property_id for now, can be expanded as needed
       matchesSearch = c.property_id.toLowerCase().includes(lowerQ) || Boolean(c.tenant_id && c.tenant_id.toLowerCase().includes(lowerQ));
    }
    
    return matchesStatus && matchesSearch;
  });

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
          <CardHeader className="pb-3">
            <div className="flex flex-col md:flex-row gap-4 justify-between items-start md:items-center">
              <div className="flex flex-wrap gap-2">
                {statusFiltersList.map(filter => (
                  <button
                    key={filter}
                    onClick={() => setStatusFilter(filter)}
                    className={`px-3 py-1.5 text-xs font-medium rounded-full transition-colors ${statusFilter === filter ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground hover:bg-muted/80'}`}
                  >
                    {filter}
                  </button>
                ))}
              </div>
              <div className="relative w-full md:w-64">
                <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                <Input 
                  placeholder="Buscar contrato..." 
                  className="pl-8" 
                  value={searchQuery}
                  onChange={e => setSearchQuery(e.target.value)}
                />
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
                  ) : filteredContracts.length === 0 ? (
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
                    filteredContracts.map((c) => (
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
                          { (c.status === 'DRAFT' || c.status === 'PENDING_SIGNATURE') && (
                            <Button
                              variant="outline"
                              size="sm"
                              className="gap-2 border-primary text-primary hover:bg-primary/10"
                              onClick={() => handleRequestSignatures(c.id)}
                            >
                              <FileText className="h-4 w-4" /> Solicitar Firmas
                            </Button>
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

      {/* Modal removido - Se usa ruta /contracts/new */}
    </div>
  )
}
