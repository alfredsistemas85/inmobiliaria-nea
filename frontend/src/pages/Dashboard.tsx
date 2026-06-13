import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Users, Home, CalendarDays, TrendingUp, DollarSign } from 'lucide-react'

const STATS = [
  { title: 'Propiedades Activas', value: '124', icon: Home, trend: '+4% este mes' },
  { title: 'Nuevos Leads', value: '32', icon: Users, trend: '+12% este mes' },
  { title: 'Citas Hoy', value: '8', icon: CalendarDays, trend: '2 confirmadas' },
  { title: 'Ventas (Mes)', value: '$2.4M', icon: DollarSign, trend: '+8% vs anterior' },
]

const RECENT_ACTIVITY = [
  { id: 1, text: 'Juan Pérez agendó una visita para Casa en San Isidro', time: 'Hace 10 min' },
  { id: 2, text: 'Nueva propiedad listada en Palermo', time: 'Hace 2 horas' },
  { id: 3, text: 'María Gómez confirmó cita de asesoramiento', time: 'Hace 3 horas' },
  { id: 4, text: 'Lead entrante desde WhatsApp', time: 'Hace 5 horas' },
]

export default function Dashboard() {
  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">Hola, John</h1>
          <p className="text-slate-500">Aquí tienes el resumen de tu actividad de hoy.</p>
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {STATS.map((stat, i) => (
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
              {RECENT_ACTIVITY.map((activity) => (
                <div key={activity.id} className="flex items-start gap-4">
                  <div className="mt-1 h-2 w-2 rounded-full bg-blue-600" />
                  <div className="space-y-1">
                    <p className="text-sm font-medium leading-none text-slate-900">{activity.text}</p>
                    <p className="text-xs text-slate-500">{activity.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
