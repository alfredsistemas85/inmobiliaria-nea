import { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Users, Home, CalendarDays, TrendingUp, DollarSign, Activity } from 'lucide-react'
import { dashboardService, DashboardStats, DashboardActivity } from '@/services/dashboard'

export default function Dashboard() {
  const [stats, setStats] = useState<DashboardStats | null>(null)
  const [activity, setActivity] = useState<DashboardActivity[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const loadData = async () => {
      try {
        const [statsRes, activityRes] = await Promise.all([
          dashboardService.getStats(),
          dashboardService.getActivity(),
        ])
        setStats(statsRes)
        setActivity(activityRes)
      } catch (error) {
        console.error('Error loading dashboard data:', error)
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [])

  if (loading || !stats) {
    return <div className="p-4 text-slate-500">Cargando métricas...</div>
  }

  const STATS_UI = [
    { title: 'Propiedades Activas', value: stats.total_properties, icon: Home, trend: 'Activas' },
    { title: 'Nuevos Leads', value: stats.new_leads, icon: Users, trend: 'Sin contactar' },
    { title: 'Citas Futuras', value: stats.upcoming_appointments, icon: CalendarDays, trend: 'Programadas' },
    { title: 'Total Clientes', value: stats.total_clients, icon: DollarSign, trend: 'Registrados' },
  ]

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Hola!</h1>
          <p className="text-slate-500">Aquí tienes el resumen de tu actividad de hoy.</p>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {STATS_UI.map((stat, i) => (
          <Card key={i}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium text-slate-600">
                {stat.title}
              </CardTitle>
              <stat.icon className="h-4 w-4 text-slate-400" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-slate-900">{stat.value}</div>
              <p className="text-xs text-slate-500 mt-1">{stat.trend}</p>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
        <Card className="lg:col-span-4">
          <CardHeader>
            <CardTitle>Rendimiento de Ventas</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px] flex items-center justify-center bg-slate-50 rounded-md border border-slate-100">
              <div className="flex flex-col items-center text-slate-400">
                <TrendingUp className="h-10 w-10 mb-2" />
                <span>Gráfico de rendimiento (Próximamente)</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="lg:col-span-3">
          <CardHeader>
            <CardTitle>Actividad Reciente</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {activity.length === 0 ? (
                <div className="text-sm text-slate-500 text-center py-4">No hay actividad reciente.</div>
              ) : (
                activity.map((act) => {
                  const dateObj = new Date(act.time)
                  const dateStr = dateObj.toLocaleDateString()
                  const timeStr = dateObj.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })

                  return (
                    <div key={act.id} className="flex items-start gap-4">
                      <div className="mt-1 h-2 w-2 rounded-full bg-blue-600 shrink-0" />
                      <div className="space-y-1">
                        <p className="text-sm font-medium leading-none text-slate-900">{act.title}</p>
                        <p className="text-xs text-slate-500">{dateStr} {timeStr} - Entidad: {act.type}</p>
                      </div>
                    </div>
                  )
                })
              )}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
