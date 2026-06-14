import { useState, useEffect } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Users, Home, CalendarDays, DollarSign, MessageCircle, BarChart3 } from 'lucide-react'
import { dashboardService, DashboardStats, DashboardActivity } from '@/services/dashboard'
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  LineChart,
  Line,
  Legend
} from 'recharts'

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884d8']

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
    { title: 'WhatsApp Activos', value: stats.active_whatsapp_conversations, icon: MessageCircle, trend: 'Abiertas' },
    { title: 'Leads del Mes', value: stats.leads_this_month, icon: BarChart3, trend: 'En curso' },
  ]

  // Data mapping for charts
  const leadsByStatusData = stats.leads_by_status || []
  const convByAgentData = stats.conversations_by_agent || []
  const conversionsByMonth = stats.conversions_by_month || []

  // Commercial Funnel approximation
  const funnelData = [
    { name: 'Nuevos', value: stats.leads_this_month },
    { name: 'En Cita', value: stats.upcoming_appointments },
    { name: 'Cerrados/Ganados', value: stats.conversions_this_month },
  ]

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Hola!</h1>
          <p className="text-slate-500">Aquí tienes el resumen de tu actividad.</p>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
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

      <div className="grid gap-4 md:grid-cols-2">
        {/* Gráfico 1: Embudo Comercial (BarChart horizontal) */}
        <Card>
          <CardHeader>
            <CardTitle>Embudo Comercial (Mes Actual)</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={funnelData} layout="vertical" margin={{ top: 5, right: 30, left: 40, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis type="number" />
                  <YAxis dataKey="name" type="category" />
                  <Tooltip />
                  <Bar dataKey="value" fill="#3b82f6" radius={[0, 4, 4, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 2: Conversión Mensual */}
        <Card>
          <CardHeader>
            <CardTitle>Conversiones Mensuales</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={conversionsByMonth} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="month" />
                  <YAxis />
                  <Tooltip />
                  <Line type="monotone" dataKey="count" name="Ganados" stroke="#10b981" strokeWidth={2} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 3: Leads por Estado */}
        <Card>
          <CardHeader>
            <CardTitle>Leads por Estado</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={leadsByStatusData}
                    cx="50%"
                    cy="50%"
                    innerRadius={60}
                    outerRadius={80}
                    paddingAngle={5}
                    dataKey="count"
                    nameKey="status"
                    label
                  >
                    {leadsByStatusData.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip />
                  <Legend />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        {/* Gráfico 4: Conversaciones por Agente */}
        <Card>
          <CardHeader>
            <CardTitle>Conversaciones Abiertas por Agente</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={convByAgentData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="agent_name" />
                  <YAxis />
                  <Tooltip />
                  <Bar dataKey="count" fill="#8b5cf6" radius={[4, 4, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>
      </div>

    </div>
  )
}
