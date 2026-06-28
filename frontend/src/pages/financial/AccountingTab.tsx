import { useState, useEffect } from 'react'
import { BookOpen, Search, Filter } from 'lucide-react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { fetchApi } from '@/services/api'

interface AccountingEntry {
  id: string
  entry_date: string
  description: string
  account_code: string
  account_name: string
  debit: string | number
  credit: string | number
  reference_id: string | null
}

export function AccountingTab() {
  const [entries, setEntries] = useState<AccountingEntry[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [searchTerm, setSearchTerm] = useState('')

  useEffect(() => {
    loadEntries()
  }, [])

  const loadEntries = async () => {
    try {
      setLoading(true)
      const data = await fetchApi('/financials/accounting/entries')
      setEntries(Array.isArray(data) ? data : data?.items || data?.data || [])
    } catch (err: any) {
      setError('Error al cargar contabilidad. ' + err.message)
    } finally {
      setLoading(false)
    }
  }

  const fmt = (val: number | string) => {
    const num = Number(val)
    if (num === 0) return '-'
    return `$${num.toLocaleString('es-AR', { minimumFractionDigits: 2 })}`
  }

  const filteredEntries = entries.filter(e => 
    e.description?.toLowerCase().includes(searchTerm.toLowerCase()) ||
    e.account_code?.includes(searchTerm) ||
    e.account_name?.toLowerCase().includes(searchTerm.toLowerCase())
  )

  // Calculate totals
  const totalDebit = filteredEntries.reduce((acc, curr) => acc + Number(curr.debit), 0)
  const totalCredit = filteredEntries.reduce((acc, curr) => acc + Number(curr.credit), 0)

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
            <CardTitle className="text-xl">Libro Diario (Ledger)</CardTitle>
            <p className="text-sm text-muted-foreground mt-1">Asientos contables por partida doble automáticos.</p>
          </div>
          <div className="flex items-center gap-2">
            <div className="relative w-64">
              <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input placeholder="Buscar cuenta o concepto..." className="pl-8" value={searchTerm} onChange={e => setSearchTerm(e.target.value)} />
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-muted/50 border-b border-border">
                <tr>
                  <th className="px-6 py-3 font-medium">Fecha</th>
                  <th className="px-6 py-3 font-medium">Cuenta</th>
                  <th className="px-6 py-3 font-medium">Concepto</th>
                  <th className="px-6 py-3 font-medium text-right">Debe</th>
                  <th className="px-6 py-3 font-medium text-right">Haber</th>
                </tr>
              </thead>
              <tbody>
                {loading ? (
                  <tr><td colSpan={5} className="px-6 py-8 text-center text-muted-foreground">Cargando libro diario...</td></tr>
                ) : filteredEntries.length === 0 ? (
                  <tr><td colSpan={5} className="px-6 py-12 text-center text-muted-foreground flex flex-col items-center gap-2">
                    <BookOpen className="h-8 w-8 opacity-20" /> No hay asientos contables registrados.
                  </td></tr>
                ) : (
                  filteredEntries.map((entry, idx) => (
                    <tr key={entry.id || idx} className="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
                      <td className="px-6 py-4 text-muted-foreground">
                        {new Date(entry.entry_date).toLocaleDateString('es-AR')}
                      </td>
                      <td className="px-6 py-4">
                        <div className="flex flex-col">
                          <span className="font-semibold text-foreground">{entry.account_code}</span>
                          <span className="text-xs text-muted-foreground">{entry.account_name}</span>
                        </div>
                      </td>
                      <td className="px-6 py-4 text-foreground">{entry.description}</td>
                      <td className="px-6 py-4 text-right font-medium">{fmt(entry.debit)}</td>
                      <td className="px-6 py-4 text-right font-medium">{fmt(entry.credit)}</td>
                    </tr>
                  ))
                )}
              </tbody>
              {!loading && filteredEntries.length > 0 && (
                <tfoot className="bg-muted/30 font-semibold border-t-2 border-border">
                  <tr>
                    <td colSpan={3} className="px-6 py-4 text-right text-muted-foreground uppercase text-xs tracking-wider">
                      Balance Total
                    </td>
                    <td className="px-6 py-4 text-right text-emerald-600">{fmt(totalDebit)}</td>
                    <td className="px-6 py-4 text-right text-emerald-600">{fmt(totalCredit)}</td>
                  </tr>
                </tfoot>
              )}
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
