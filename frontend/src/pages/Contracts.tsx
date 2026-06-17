import { useState, useEffect } from 'react'
import { Plus, Search, Download, CheckCircle, XCircle } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'

export default function Contracts() {
  const [activeTab, setActiveTab] = useState<'contracts' | 'pending'>('contracts')
  const [contracts, setContracts] = useState<any[]>([])
  const [pendingAdjustments, setPendingAdjustments] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [toastMessage, setToastMessage] = useState<string | null>(null)

  useEffect(() => {
    if (activeTab === 'contracts') {
      loadContracts()
    } else {
      loadPendingAdjustments()
    }
  }, [activeTab])

  const showToast = (msg: string) => {
    setToastMessage(msg)
    setTimeout(() => setToastMessage(null), 3000)
  }

  const loadContracts = async () => {
    try {
      setLoading(true)
      const res = await authService.fetchApi('/contracts')
      const data = await res.json()
      setContracts(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const loadPendingAdjustments = async () => {
    try {
      setLoading(true)
      const res = await authService.fetchApi('/contracts/adjustments/pending')
      const data = await res.json()
      setPendingAdjustments(data.items || [])
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleApprove = async (id: string) => {
    try {
      const res = await authService.fetchApi(`/contracts/adjustments/${id}/approve`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ new_amount: null, notes: "Aprobado manualmente" }) // will use calculated amount from engine if new_amount is null
      })
      if (res.ok) {
        showToast("Ajuste aprobado correctamente.")
        loadPendingAdjustments()
      } else {
        showToast("Error al aprobar el ajuste.")
      }
    } catch (err) {
      console.error(err)
      showToast("Error de conexión.")
    }
  }

  const handleReject = async (id: string) => {
    const reason = window.prompt("Motivo de rechazo:")
    if (reason === null) return // cancelled
    try {
      const res = await authService.fetchApi(`/contracts/adjustments/${id}/reject`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ reason })
      })
      if (res.ok) {
        showToast("Ajuste rechazado.")
        loadPendingAdjustments()
      } else {
        showToast("Error al rechazar el ajuste.")
      }
    } catch (err) {
      console.error(err)
      showToast("Error de conexión.")
    }
  }

  return (
    <div className="space-y-6 relative">
      {toastMessage && (
        <div className="fixed bottom-4 right-4 bg-green-600 text-white px-4 py-2 rounded-md shadow-lg z-50 transition-opacity">
          {toastMessage}
        </div>
      )}

      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Contratos</h1>
          <p className="text-muted-foreground">Gestión de contratos de alquiler y vencimientos.</p>
        </div>
        <Button className="gap-2">
          <Plus className="h-4 w-4" /> Nuevo Contrato
        </Button>
      </div>

      <div className="flex space-x-4 border-b border-border">
        <button
          className={`pb-2 px-1 text-sm font-medium transition-colors border-b-2 ${
            activeTab === 'contracts'
              ? 'border-primary text-primary'
              : 'border-transparent text-muted-foreground hover:text-foreground hover:border-border'
          }`}
          onClick={() => setActiveTab('contracts')}
        >
          Listado de Contratos
        </button>
        <button
          className={`pb-2 px-1 text-sm font-medium transition-colors border-b-2 flex items-center gap-2 ${
            activeTab === 'pending'
              ? 'border-primary text-primary'
              : 'border-transparent text-muted-foreground hover:text-foreground hover:border-border'
          }`}
          onClick={() => setActiveTab('pending')}
        >
          Ajustes Pendientes
        </button>
      </div>

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
            <div className="rounded-md border">
              <table className="w-full text-sm text-left">
                <thead className="text-xs text-muted-foreground uppercase bg-muted/50">
                  <tr>
                    <th className="px-6 py-3 font-medium">Propiedad</th>
                    <th className="px-6 py-3 font-medium">Inquilino</th>
                    <th className="px-6 py-3 font-medium">Monto</th>
                    <th className="px-6 py-3 font-medium">Vencimiento</th>
                    <th className="px-6 py-3 font-medium">Estado</th>
                    <th className="px-6 py-3 font-medium">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  {contracts.length === 0 ? (
                    <tr>
                      <td colSpan={6} className="px-6 py-8 text-center text-muted-foreground">
                        {loading ? 'Cargando...' : 'No hay contratos registrados.'}
                      </td>
                    </tr>
                  ) : (
                    contracts.map((contract) => (
                      <tr key={contract.id} className="border-b last:border-0 hover:bg-muted/50">
                        <td className="px-6 py-4">{contract.property_id}</td>
                        <td className="px-6 py-4">{contract.tenant_user_id}</td>
                        <td className="px-6 py-4">${contract.rent_amount}</td>
                        <td className="px-6 py-4">{contract.end_date}</td>
                        <td className="px-6 py-4">
                          <span className="px-2.5 py-1 rounded-full text-xs font-medium bg-green-100 text-green-700">
                            {contract.status || 'Activo'}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          <Button variant="ghost" size="sm" className="gap-2" onClick={() => window.open(`http://localhost:3000/api/contracts/${contract.id}/pdf`, '_blank')}>
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

      {activeTab === 'pending' && (
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Ajustes Pendientes de Aprobación</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border">
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
                  {pendingAdjustments.length === 0 ? (
                    <tr>
                      <td colSpan={7} className="px-6 py-8 text-center text-muted-foreground">
                        {loading ? 'Cargando...' : 'No hay ajustes pendientes.'}
                      </td>
                    </tr>
                  ) : (
                    pendingAdjustments.map((adj) => (
                      <tr key={adj.adjustment_id} className="border-b last:border-0 hover:bg-muted/50">
                        <td className="px-6 py-4">{adj.contract_number}</td>
                        <td className="px-6 py-4">{adj.tenant_name}</td>
                        <td className="px-6 py-4">${adj.current_rent}</td>
                        <td className="px-6 py-4">{adj.adjustment_percent}%</td>
                        <td className="px-6 py-4 font-semibold text-primary">${adj.new_rent}</td>
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
    </div>
  )
}
