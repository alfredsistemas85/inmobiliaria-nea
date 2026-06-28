import React from 'react';
import { useFormContext } from 'react-hook-form';
import { ContractFormValues } from '../schema';
import { Input } from '@/components/ui/input';

export const TermsStep = React.memo(function TermsStep() {
  const { register } = useFormContext<ContractFormValues>();

  return (
    <div className="space-y-6">
      <h3 className="text-lg font-semibold text-white border-b border-zinc-800 pb-2">Condiciones (Términos)</h3>
      <div className="grid grid-cols-2 gap-4">
        <label className="flex items-center space-x-2 text-white">
          <input type="checkbox" {...register('terms.allows_pets')} />
          <span>Permite Mascotas</span>
        </label>
        <label className="flex items-center space-x-2 text-white">
          <input type="checkbox" {...register('terms.allows_sublease')} />
          <span>Permite Subalquiler</span>
        </label>
        <label className="flex items-center space-x-2 text-white">
          <input type="checkbox" {...register('terms.requires_inventory')} />
          <span>Inventario Obligatorio</span>
        </label>
        <label className="flex items-center space-x-2 text-white">
          <input type="checkbox" {...register('terms.requires_insurance')} />
          <span>Seguro Obligatorio</span>
        </label>
        <label className="flex items-center space-x-2 text-white">
          <input type="checkbox" {...register('terms.automatic_renewal')} />
          <span>Renovación Automática</span>
        </label>
      </div>

      <div className="grid grid-cols-2 gap-4 mt-4">
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Días Preaviso (Rescisión)</label>
          <Input type="number" {...register('terms.notice_days')} />
        </div>
        <div>
          <label className="block text-sm text-zinc-400 mb-1">Actividad Permitida</label>
          <Input {...register('terms.permitted_activity')} />
        </div>
      </div>
    </div>
  );
});
