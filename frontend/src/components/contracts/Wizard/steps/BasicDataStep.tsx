import React, { useEffect, useState } from 'react';
import { useFormContext } from 'react-hook-form';
import { ContractFormValues } from '../schema';
import { Input } from '@/components/ui/input';
import { propertiesService } from '@/services/properties';
import { useWizardContext } from '../WizardProvider';

export const BasicDataStep = React.memo(function BasicDataStep() {
  const { register, formState: { errors }, setValue } = useFormContext<ContractFormValues>();
  const { isPropertyCentric } = useWizardContext();
  const [properties, setProperties] = useState<any[]>([]);

  useEffect(() => {
    if (!isPropertyCentric) {
      propertiesService.getAll(100, 0).then(data => setProperties(Array.isArray(data) ? data : data?.data || []));
    }
  }, [isPropertyCentric]);

  return (
    <div className="space-y-4">
      <h3 className="text-lg text-white font-medium">Paso 1: Datos Básicos</h3>
      <div className="grid grid-cols-2 gap-4">
        {!isPropertyCentric && (
          <div>
            <label className="block text-sm text-zinc-400 mb-1">Propiedad</label>
            <select
              className={`w-full bg-zinc-800 border ${errors.property_id ? 'border-red-500' : 'border-zinc-700'} rounded-lg px-4 py-2 text-white`}
              {...register('property_id')}
            >
              <option value="">Seleccionar...</option>
              {properties.map(p => (
                <option key={p.id} value={p.id}>{p.title}</option>
              ))}
            </select>
            {errors.property_id && <span className="text-xs text-red-500">{errors.property_id.message}</span>}
          </div>
        )}
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Monto Inicial</label>
          <Input 
            type="number"
            {...register('original_rent_amount')}
            className={errors.original_rent_amount ? 'border-red-500' : ''}
            disabled={isPropertyCentric}
          />
          {errors.original_rent_amount && <span className="text-xs text-red-500">{errors.original_rent_amount.message}</span>}
        </div>
        {!isPropertyCentric && (
          <div>
            <label className="block text-sm text-zinc-400 mb-1">Moneda</label>
            <select
              className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
              {...register('currency')}
            >
              <option value="ARS">ARS</option>
              <option value="USD">USD</option>
            </select>
          </div>
        )}
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Fecha Inicio</label>
          <Input 
            type="date"
            {...register('start_date')}
            className={errors.start_date ? 'border-red-500' : ''}
          />
          {errors.start_date && <span className="text-xs text-red-500">{errors.start_date.message}</span>}
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Fecha Fin</label>
          <Input 
            type="date"
            {...register('end_date')}
            className={errors.end_date ? 'border-red-500' : ''}
          />
          {errors.end_date && <span className="text-xs text-red-500">{errors.end_date.message}</span>}
        </div>
        
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Tipo</label>
          <select
            className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
            {...register('c_type')}
            disabled={isPropertyCentric}
          >
            <option value="HOUSING">Vivienda</option>
            <option value="COMMERCIAL">Comercial</option>
            <option value="TEMPORARY">Temporal</option>
          </select>
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Destino</label>
          <select
            className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
            {...register('c_destination')}
            disabled={isPropertyCentric}
          >
            <option value="HABITATIONAL">Habitacional</option>
            <option value="COMMERCIAL">Comercial</option>
          </select>
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Método de Ajuste</label>
          <select
            className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
            {...register('adjustment_method')}
          >
            <option value="IPC">IPC</option>
            <option value="ICL">ICL</option>
            <option value="FIXED_PERCENTAGE">Porcentaje Fijo</option>
          </select>
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Frecuencia de Ajuste</label>
          <select
            className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
            {...register('adjustment_frequency')}
          >
            <option value="MONTHLY">Mensual</option>
            <option value="BIMONTHLY">Bimestral</option>
            <option value="QUARTERLY">Trimestral</option>
            <option value="SEMESTER">Semestral</option>
            <option value="ANNUAL">Anual</option>
          </select>
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Automatización</label>
          <select
            className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
            {...register('automation_mode')}
          >
            <option value="MANUAL">Manual</option>
            <option value="SEMIAUTOMATIC">Semiautomático (Sugerir)</option>
            <option value="AUTOMATIC">Automático</option>
          </select>
        </div>
      </div>
    </div>
  );
});
