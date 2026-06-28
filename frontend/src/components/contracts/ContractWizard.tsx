import { useState, useEffect } from 'react';
import { X, Plus, Trash2 } from 'lucide-react';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { clientsService, Client } from '@/services/clients';
import { propertiesService } from '@/services/properties';
import { fetchApi } from '@/services/api';
import { useForm, useFieldArray } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

// ─── ZOD SCHEMAS ─────────────────────────────────────────────────────────────
const guaranteeSchema = z.object({
  guarantee_type: z.string(),
  income_amount: z.coerce.number().min(0),
  employer: z.string().optional(),
  guarantee_details: z.string().optional()
});

const participantSchema = z.object({
  client_id: z.string().min(1, "Selecciona un cliente").uuid("ID inválido"),
  p_role: z.string(),
  percentage: z.coerce.number().min(0, "Mín. 0").max(100, "Máx. 100"),
  is_main: z.boolean(),
  guarantees: z.array(guaranteeSchema).default([])
});

const clauseSchema = z.object({
  title: z.string().default(""),
  body: z.string().default(""),
  is_mandatory: z.boolean().default(false),
  is_editable: z.boolean().default(true),
  is_system: z.boolean().default(false)
});

const contractSchema = z.object({
  property_id: z.string().min(1, "Debes seleccionar una propiedad").uuid(),
  start_date: z.string().min(1, "Requerido"),
  end_date: z.string().min(1, "Requerido"),
  original_rent_amount: z.coerce.number().min(1, "Debe ser mayor a 0"),
  adjustment_method: z.string(),
  adjustment_frequency: z.string(),
  automation_mode: z.string(),
  fixed_percentage: z.coerce.number().nullable().optional(),
  notification_days_before: z.coerce.number().min(0),
  c_type: z.string(),
  c_destination: z.string(),
  currency: z.string(),
  deposit_amount: z.coerce.number().min(0),
  
  participants: z.array(participantSchema),
  
  terms: z.object({
    allows_pets: z.boolean(),
    allows_sublease: z.boolean(),
    requires_inventory: z.boolean(),
    requires_insurance: z.boolean(),
    automatic_renewal: z.boolean(),
    permitted_activity: z.string().optional(),
    notice_days: z.coerce.number().min(0),
    early_termination_penalty: z.string().optional(),
    observations: z.string().optional()
  }),
  
  clauses: z.array(clauseSchema),
  template_id: z.string().nullable().optional()
});

type ContractFormValues = z.infer<typeof contractSchema>;

interface ContractWizardProps {
  onClose: () => void;
  onSuccess: () => void;
}

export default function ContractWizard({ onClose, onSuccess }: ContractWizardProps) {
  const [step, setStep] = useState(1);
  const [globalError, setGlobalError] = useState('');
  
  const [clients, setClients] = useState<Client[]>([]);
  const [properties, setProperties] = useState<any[]>([]);
  const [templates, setTemplates] = useState<any[]>([]);

  // ─── RHF SETUP ─────────────────────────────────────────────────────────────
  const { register, control, handleSubmit, watch, setValue, trigger, formState: { errors, isSubmitting } } = useForm<ContractFormValues>({
    resolver: zodResolver(contractSchema) as any,
    defaultValues: {
      property_id: '',
      start_date: '',
      end_date: '',
      original_rent_amount: 0,
      adjustment_method: 'IPC',
      adjustment_frequency: 'QUARTERLY',
      automation_mode: 'SEMIAUTOMATIC',
      fixed_percentage: null,
      notification_days_before: 30,
      c_type: 'HOUSING',
      c_destination: 'HABITATIONAL',
      currency: 'ARS',
      deposit_amount: 0,
      participants: [],
      terms: {
        allows_pets: false,
        allows_sublease: false,
        requires_inventory: false,
        requires_insurance: false,
        automatic_renewal: false,
        permitted_activity: '',
        notice_days: 30,
        early_termination_penalty: '',
        observations: ''
      },
      clauses: [],
      template_id: null
    }
  });

  const { fields: participantsFields, append: appendParticipant, remove: removeParticipant } = useFieldArray({
    control,
    name: "participants"
  });

  const { fields: clausesFields, append: appendClause, remove: removeClause, replace: replaceClauses } = useFieldArray({
    control,
    name: "clauses"
  });

  const currentPropertyId = watch('property_id');
  const currentParticipants = watch('participants');
  const currentTemplateId = watch('template_id');

  useEffect(() => {
    propertiesService.getAll(100, 0).then(data => setProperties(Array.isArray(data) ? data : data?.data || []));
    clientsService.getClients(100).then(res => setClients(res.data || []));
    fetchApi('/contracts/v2/contract-templates').then(data => setTemplates(data || []));
  }, []);

  // When property changes, auto-add landlords
  useEffect(() => {
    if (currentPropertyId) {
      const prop = properties.find(p => p.id === currentPropertyId);
      if (prop && prop.owners && prop.owners.length > 0) {
        // Remove existing landlords first
        const nonLandlords = currentParticipants.filter(p => p.p_role !== 'LANDLORD');
        setValue('participants', nonLandlords);
        
        prop.owners.forEach((o: any) => {
          appendParticipant({
            client_id: o.client_id,
            p_role: 'LANDLORD',
            percentage: o.percentage,
            is_main: true,
            guarantees: []
          });
        });
      }
    }
  }, [currentPropertyId, properties, appendParticipant, setValue]);

  // When template changes, load clauses
  useEffect(() => {
    async function loadTemplate() {
      if (!currentTemplateId) {
        replaceClauses([]);
        return;
      }
      try {
        const templateData = await fetchApi(`/contracts/v2/contract-templates/${currentTemplateId}`);
        if (templateData && templateData.clauses) {
          replaceClauses(templateData.clauses);
        }
      } catch (err) {
        console.error(err);
      }
    }
    loadTemplate();
  }, [currentTemplateId, replaceClauses]);

  const addParticipant = (role: string, is_main: boolean) => {
    appendParticipant({
      client_id: '',
      p_role: role,
      percentage: 100,
      is_main,
      guarantees: []
    });
  };

  const addGuarantee = (pIndex: number) => {
    const parts = [...watch('participants')];
    if (!parts[pIndex].guarantees) parts[pIndex].guarantees = [];
    parts[pIndex].guarantees.push({
      guarantee_type: 'PAYSLIP',
      income_amount: 0,
      employer: '',
      guarantee_details: ''
    });
    setValue('participants', parts);
  };

  const removeGuarantee = (pIndex: number, gIndex: number) => {
    const parts = [...watch('participants')];
    parts[pIndex].guarantees.splice(gIndex, 1);
    setValue('participants', parts);
  };

  const handleNext = async () => {
    setGlobalError('');
    let fieldsToValidate: (keyof ContractFormValues)[] = [];
    
    if (step === 1) fieldsToValidate = ['property_id', 'start_date', 'end_date', 'original_rent_amount', 'adjustment_method'];
    if (step === 2 || step === 3) fieldsToValidate = ['participants'];
    
    const isValid = await trigger(fieldsToValidate as any);
    
    if (!isValid) {
      setGlobalError('Por favor, revisa los campos marcados en rojo antes de continuar.');
      return;
    }

    if (step === 2 || step === 3) {
      const parts = watch('participants');
      const hasLandlord = parts.some(p => p.p_role === 'LANDLORD' && p.is_main);
      const hasTenant = parts.some(p => p.p_role === 'TENANT' && p.is_main);
      if (!hasLandlord || !hasTenant) {
         setGlobalError('Debes incluir al menos un locador principal y un locatario principal.');
         return;
      }
    }

    setStep(step + 1);
  };

  const onSubmit = async (data: ContractFormValues) => {
    try {
      setGlobalError('');
      
      const payload = {
        ...data,
        status: 'DRAFT',
        commission_amount: 0,
        fees_amount: 0,
        clauses: data.clauses.map((c, i) => ({ ...c, display_order: i + 1 })),
        // Garantizar que template_id sea null si es string vacío
        template_id: data.template_id || null
      };

      await fetchApi('/contracts/v2', {
        method: 'POST',
        body: JSON.stringify(payload)
      });
      
      onSuccess();
    } catch (err: any) {
      setGlobalError(err.message || 'Error al crear el contrato');
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 overflow-y-auto">
      <div className="bg-zinc-900 border border-zinc-800 rounded-xl shadow-2xl w-full max-w-4xl flex flex-col max-h-[90vh]">
        <div className="flex justify-between items-center p-6 border-b border-zinc-800">
          <h2 className="text-xl font-semibold text-white">Nuevo Contrato V2 (Validado)</h2>
          <button onClick={onClose} className="text-zinc-400 hover:text-white">
            <X size={24} />
          </button>
        </div>

        <div className="p-6 overflow-y-auto flex-1">
          {globalError && <div className="mb-4 p-3 bg-red-500/20 text-red-400 rounded-lg">{globalError}</div>}
          
          <div className="flex mb-6 space-x-2">
            {[1, 2, 3, 4, 5, 6].map(s => (
              <div key={s} className={`h-2 flex-1 rounded-full ${step >= s ? 'bg-cyan-500' : 'bg-zinc-800'}`} />
            ))}
          </div>

          <form onSubmit={e => e.preventDefault()}>
            {step === 1 && (
              <div className="space-y-4">
                <h3 className="text-lg text-white font-medium">Paso 1: Datos Básicos</h3>
                <div className="grid grid-cols-2 gap-4">
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
                  <div>
                    <label className="block text-sm text-zinc-400 mb-1">Monto Inicial (ARS)</label>
                    <Input 
                      type="number"
                      {...register('original_rent_amount')}
                      className={errors.original_rent_amount ? 'border-red-500' : ''}
                    />
                    {errors.original_rent_amount && <span className="text-xs text-red-500">{errors.original_rent_amount.message}</span>}
                  </div>
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
                  
                  {/* Nuevos inputs para variables comerciales */}
                  <div>
                    <label className="block text-sm text-zinc-400 mb-1">Tipo</label>
                    <select
                      className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                      {...register('c_type')}
                    >
                      <option value="HOUSING">Vivienda</option>
                      <option value="COMMERCIAL">Comercial</option>
                      <option value="TEMPORARY">Temporal</option>
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
            )}

            {step === 2 && (
              <div className="space-y-4">
                <div className="flex justify-between items-center">
                  <h3 className="text-lg text-white font-medium">Paso 2: Participantes (Locadores / Locatarios)</h3>
                  <div className="space-x-2">
                    <Button type="button" onClick={() => addParticipant('TENANT', true)} variant="outline" size="sm">
                      <Plus size={16} className="mr-1"/> Locatario
                    </Button>
                  </div>
                </div>
                
                {participantsFields.map((field, i) => {
                  const pRole = watch(`participants.${i}.p_role`);
                  const isLandlord = pRole === 'LANDLORD';
                  
                  if (pRole === 'GUARANTOR') return null; // Los mostramos en el paso 3
                  
                  return (
                  <Card key={field.id} className={`bg-zinc-800 ${isLandlord ? 'border-cyan-700/50' : 'border-zinc-700'}`}>
                    <CardContent className="p-4 flex gap-4 items-end">
                      <div className="flex-1">
                        <label className="block text-sm text-zinc-400 mb-1">
                          {isLandlord ? 'Propietario / Locador (Automático)' : `Cliente (${pRole})`}
                        </label>
                        {isLandlord ? (
                          <div className="w-full bg-zinc-900/50 border border-zinc-700/50 rounded-lg px-4 py-2 text-zinc-300 flex items-center justify-between">
                            <span>
                              {(() => {
                                const clientId = watch(`participants.${i}.client_id`);
                                const c = clients.find(c => c.id === clientId);
                                return c ? `${c.first_name} ${c.last_name}` : 'Cargando...';
                              })()}
                            </span>
                            <span className="text-xs text-cyan-500 font-medium">Desde Propiedad</span>
                          </div>
                        ) : (
                          <>
                            <select
                              className={`w-full bg-zinc-900 border ${errors.participants?.[i]?.client_id ? 'border-red-500' : 'border-zinc-700'} rounded-lg px-4 py-2 text-white`}
                              {...register(`participants.${i}.client_id`)}
                            >
                              <option value="">Seleccionar cliente...</option>
                              {clients.map(c => (
                                <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                              ))}
                            </select>
                            {errors.participants?.[i]?.client_id && <span className="text-xs text-red-500">{errors.participants?.[i]?.client_id?.message}</span>}
                          </>
                        )}
                      </div>
                      <div>
                        <label className="block text-sm text-zinc-400 mb-1">% Part.</label>
                        {isLandlord ? (
                          <div className="w-24 bg-zinc-900/50 border border-zinc-700/50 rounded-lg px-4 py-2 text-zinc-300 text-center">
                            {watch(`participants.${i}.percentage`)}
                          </div>
                        ) : (
                          <Input type="number" {...register(`participants.${i}.percentage`)} className="w-24" />
                        )}
                      </div>
                      {!isLandlord && (
                        <Button type="button" variant="ghost" onClick={() => removeParticipant(i)} className="text-red-400 hover:text-red-300">
                          <Trash2 size={20} />
                        </Button>
                      )}
                    </CardContent>
                  </Card>
                  );
                })}
              </div>
            )}

            {step === 3 && (
              <div className="space-y-4">
                <div className="flex justify-between items-center">
                  <h3 className="text-lg text-white font-medium">Paso 3: Garantes</h3>
                  <Button type="button" onClick={() => addParticipant('GUARANTOR', false)} variant="outline" size="sm">
                    <Plus size={16} className="mr-1"/> Agregar Garante
                  </Button>
                </div>
                
                <div className="text-sm text-zinc-400 mb-4">Los garantes respaldan las obligaciones del locatario.</div>

                {participantsFields.map((field, i) => {
                  if (watch(`participants.${i}.p_role`) !== 'GUARANTOR') return null;
                  
                  return (
                    <Card key={field.id} className="bg-zinc-800 border-zinc-700 mb-4">
                      <CardHeader className="p-4 pb-0 flex flex-row justify-between items-center">
                        <div className="font-medium text-white">Garante</div>
                        <div className="space-x-2">
                          <Button type="button" onClick={() => addGuarantee(i)} size="sm" variant="outline">
                            <Plus size={16} className="mr-1"/> Añadir Respaldo
                          </Button>
                          <Button type="button" onClick={() => removeParticipant(i)} size="sm" variant="ghost" className="text-red-400">
                            <Trash2 size={16} />
                          </Button>
                        </div>
                      </CardHeader>
                      <CardContent className="p-4 space-y-4">
                        <div>
                          <label className="block text-sm text-zinc-400 mb-1">Cliente Garante</label>
                          <select
                            className={`w-full bg-zinc-900 border ${errors.participants?.[i]?.client_id ? 'border-red-500' : 'border-zinc-700'} rounded-lg px-4 py-2 text-white`}
                            {...register(`participants.${i}.client_id`)}
                          >
                            <option value="">Seleccionar garante...</option>
                            {clients.map(c => (
                              <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                            ))}
                          </select>
                          {errors.participants?.[i]?.client_id && <span className="text-xs text-red-500">{errors.participants?.[i]?.client_id?.message}</span>}
                        </div>

                        {watch(`participants.${i}.guarantees`)?.map((_, gIndex) => (
                          <div key={gIndex} className="grid grid-cols-3 gap-4 p-3 bg-zinc-900 rounded-lg border border-zinc-700">
                            <div>
                              <label className="block text-sm text-zinc-400 mb-1">Tipo de Respaldo</label>
                              <select
                                className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                                {...register(`participants.${i}.guarantees.${gIndex}.guarantee_type`)}
                              >
                                <option value="PAYSLIP">Recibo de Sueldo</option>
                                <option value="PROPERTY">Propietario</option>
                                <option value="SURETY_BOND">Seguro de Caución</option>
                              </select>
                            </div>
                            <div className="col-span-2 flex gap-2 items-end">
                              <div className="flex-1">
                                <label className="block text-sm text-zinc-400 mb-1">Detalle / Empleador</label>
                                <Input 
                                  {...register(`participants.${i}.guarantees.${gIndex}.employer`)}
                                />
                              </div>
                              <Button type="button" variant="ghost" onClick={() => removeGuarantee(i, gIndex)} className="text-red-400">
                                <Trash2 size={16} />
                              </Button>
                            </div>
                          </div>
                        ))}
                      </CardContent>
                    </Card>
                  );
                })}
              </div>
            )}

            {step === 4 && (
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
            )}

            {step === 5 && (
              <div className="space-y-6">
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
            )}

            {step === 6 && (
              <div className="space-y-6">
                <div className="flex justify-between items-center border-b border-zinc-800 pb-2">
                  <h3 className="text-lg font-semibold text-white">Cláusulas del Contrato</h3>
                  <Button type="button" size="sm" onClick={() => appendClause({ title: '', body: '', is_mandatory: false, is_editable: true, is_system: false })}>
                    <Plus size={16} className="mr-1"/> Agregar Cláusula Libre
                  </Button>
                </div>
                
                <div className="space-y-4">
                  {clausesFields.map((field, idx) => (
                    <Card key={field.id} className="bg-zinc-800 border-zinc-700">
                      <CardHeader className="p-3 pb-0 flex flex-row justify-between items-center">
                        <Input 
                          placeholder="Título de cláusula" 
                          disabled={!watch(`clauses.${idx}.is_editable`)}
                          {...register(`clauses.${idx}.title`)}
                          className="font-medium bg-transparent border-none text-white focus-visible:ring-0 max-w-[300px]"
                        />
                        {!watch(`clauses.${idx}.is_mandatory`) && (
                          <Button type="button" variant="ghost" size="sm" onClick={() => removeClause(idx)} className="text-red-400 hover:text-red-300">
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
                  {clausesFields.length === 0 && <div className="text-zinc-500 text-sm">No hay cláusulas definidas.</div>}
                </div>
              </div>
            )}
          </form>
        </div>

        <div className="p-6 border-t border-zinc-800 flex justify-between">
          <Button type="button" variant="ghost" onClick={step > 1 ? () => setStep(step - 1) : onClose}>
            {step > 1 ? 'Atrás' : 'Cancelar'}
          </Button>
          
          {step < 6 ? (
            <Button type="button" onClick={handleNext}>Siguiente</Button>
          ) : (
            <Button type="button" onClick={handleSubmit(onSubmit as any)} disabled={isSubmitting}>
              {isSubmitting ? 'Creando...' : 'Finalizar y Crear Contrato'}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
