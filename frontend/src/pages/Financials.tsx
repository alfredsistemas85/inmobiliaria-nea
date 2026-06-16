import { useState, useEffect } from 'react'
import { DollarSign, Search, CheckCircle, CreditCard } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { authService } from '@/services/auth'

export default function Financials() {
  const [activeTab, setActiveTab] = useState<'invoices' | 'liquidations'>('invoices')
  const [invoices, setInvoices] = useState<any[]>([])
  const [liquidations, setLiquidations] = useState<any[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    if (activeTab === 'invoices') {
      loadInvoices()
    } else {
      loadLiquidations()
    }
  }, [activeTab])

  const loadInvoices = async () => {
    try {
      setLoading(true)
      const res = await authService.fetchApi('/financials/invoices')
      const data = await res.json()
      setInvoices(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const loadLiquidations = async () => {
    try {
      setLoading(true)
      const res = await authService.fetchApi('/financials/liquidations')
      const data = await res.json()
      setLiquidations(data)
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const markAsPaid = async (id: string) => {
    try {
      await authService.fetchApi(`/financials/invoices/${id}/pay_manual`, { method: 'POST' })
      loadInvoices()
    } catch (err) {
      alert("Error al marcar como pagado")
    }
  }

  const payWithMP = async (id: string) => {
    try {
      const res = await authService.fetchApi(`/payments/checkout/rent/${id}`, { method: 'POST' })
      const data = await res.json()
      if (data.init_point) {
        window.location.href = data.init_point
      }
    } catch (err) {
      alert("Error al generar pago")
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Finanzas</h1>
          <p className="text-muted-foreground">Cobro de alquileres y liquidaciones a propietarios.</p>
        </div>
        <Button className="gap-2">
          <DollarSign className="h-4 w-4" /> Generar Cobro Mensual
        </Button>
      </div>

      <div className="flex gap-4 border-b">
        <button
          className={`pb-2 px-1 font-medium text-sm transition-colors ${activeTab === 'invoices' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
          onClick={() => setActiveTab('invoices')}
        >
          Alquileres / Expensas
        </button>
        <button
          className={`pb-2 px-1 font-medium text-sm transition-colors ${activeTab === 'liquidations' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
          onClick={() => setActiveTab('liquidations')}
        >
          Liquidaciones a Propietarios
        </button>
      </div>

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
          <div className="rounded-md border">
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
                {activeTab === 'invoices' ? (
                  invoices.length === 0 ? (
                    <tr><td colSpan={6} className="px-6 py-8 text-center text-muted-foreground">No hay cobros registrados.</td></tr>
                  ) : (
                    invoices.map((inv) => (
                      <tr key={inv.id} className="border-b last:border-0 hover:bg-muted/50">
                        <td className="px-6 py-4">{inv.contract_id}</td>
                        <td className="px-6 py-4 font-medium">${inv.amount}</td>
                        <td className="px-6 py-4">${inv.commission || '0'}</td>
                        <td className="px-6 py-4">{inv.due_date}</td>
                        <td className="px-6 py-4">
                          <span className={`px-2.5 py-1 rounded-full text-xs font-medium ${inv.status === 'PAID' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'}`}>
                            {inv.status || 'PENDING'}
                          </span>
                        </td>
                        <td className="px-6 py-4 flex gap-2">
                          {inv.status !== 'PAID' && (
                            <>
                              <Button size="sm" variant="outline" className="gap-2" onClick={() => markAsPaid(inv.id)}>
                                <CheckCircle className="h-4 w-4" /> Manual
                              </Button>
                              <Button size="sm" className="gap-2" onClick={() => payWithMP(inv.id)}>
                                <CreditCard className="h-4 w-4" /> MP
                              </Button>
                            </>
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
                        <td className="px-6 py-4">${liq.total_collected}</td>
                        <td className="px-6 py-4 text-red-600">-${liq.commission_deducted}</td>
                        <td className="px-6 py-4 font-bold text-green-700">${liq.net_to_transfer}</td>
                      </tr>
                    ))
                  )
                )}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
