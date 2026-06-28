import React, { useState, useEffect } from 'react';
import { useFormContext, useFieldArray } from 'react-hook-form';
import { Plus, Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { ContractFormValues } from '../schema';
import { fetchApi } from '@/services/api';

export const ClausesStep = React.memo(function ClausesStep() {
  const { control, register, watch } = useFormContext<ContractFormValues>();
  const [templates, setTemplates] = useState<any[]>([]);

  const { fields, append, remove, replace } = useFieldArray({
    control,
    name: "clauses"
  });

  const currentTemplateId = watch('template_id');

  useEffect(() => {
    fetchApi('/contracts/v2/contract-templates').then(data => setTemplates(data || []));
  }, []);

  useEffect(() => {
    async function loadTemplate() {
      if (!currentTemplateId) {
        replace([]);
        return;
      }
      try {
        const templateData = await fetchApi(`/contracts/v2/contract-templates/${currentTemplateId}`);
        if (templateData && templateData.clauses) {
          replace(templateData.clauses);
        }
      } catch (err) {
        console.error(err);
      }
    }
    loadTemplate();
  }, [currentTemplateId, replace]);

  return (
    <div className="space-y-6">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold text-white border-b border-zinc-800 pb-2">Plantilla</h3>
        <p className="text-sm text-zinc-400 mb-4">Selecciona una plantilla para cargar automáticamente las cláusulas.</p>
        
        <select
          className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white mb-6"
          {...register('template_id')}
        >
          <option value="">Sin Plantilla (Contrato en Blanco)</option>
          {templates.map(t => (
            <option key={t.id} value={t.id}>{t.name}</option>
          ))}
        </select>
      </div>

      <div className="space-y-6">
        <div className="flex justify-between items-center border-b border-zinc-800 pb-2">
          <h3 className="text-lg font-semibold text-white">Cláusulas del Contrato</h3>
          <Button type="button" size="sm" onClick={() => append({ title: '', body: '', is_mandatory: false, is_editable: true, is_system: false })}>
            <Plus size={16} className="mr-1"/> Agregar Cláusula Libre
          </Button>
        </div>
        
        <div className="space-y-4">
          {fields.map((field, idx) => (
            <Card key={field.id} className="bg-zinc-800 border-zinc-700">
              <CardHeader className="p-3 pb-0 flex flex-row justify-between items-center">
                <Input 
                  placeholder="Título de cláusula" 
                  disabled={!watch(`clauses.${idx}.is_editable`)}
                  {...register(`clauses.${idx}.title`)}
                  className="font-medium bg-transparent border-none text-white focus-visible:ring-0 max-w-[300px]"
                />
                {!watch(`clauses.${idx}.is_mandatory`) && (
                  <Button type="button" variant="ghost" size="sm" onClick={() => remove(idx)} className="text-red-400 hover:text-red-300">
                    <Trash2 size={16} />
                  </Button>
                )}
              </CardHeader>
              <CardContent className="p-3">
                <textarea 
                  className="w-full bg-zinc-900 border border-zinc-700 rounded-lg p-3 text-white text-sm min-h-[100px]"
                  disabled={!watch(`clauses.${idx}.is_editable`)}
                  {...register(`clauses.${idx}.body`)}
                  placeholder="Redacta la cláusula aquí..."
                />
                <div className="mt-2 flex space-x-4 text-xs text-zinc-500">
                  {watch(`clauses.${idx}.is_mandatory`) && <span>🔒 Obligatoria (No se puede eliminar)</span>}
                  {!watch(`clauses.${idx}.is_editable`) && <span>🔒 No editable</span>}
                </div>
              </CardContent>
            </Card>
          ))}
          {fields.length === 0 && <div className="text-zinc-500 text-sm">No hay cláusulas definidas.</div>}
        </div>
      </div>
    </div>
  );
}
