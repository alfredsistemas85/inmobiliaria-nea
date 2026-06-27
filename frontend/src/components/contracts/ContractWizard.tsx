import React, { useState, useEffect } from 'react';
import { X, Search, Plus, Trash2 } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { clientsService, Client } from '@/services/clients';
import { propertiesService } from '@/services/properties';
import { fetchApi } from '@/services/api';

interface ContractWizardProps {
  onClose: () => void;
  onSuccess: () => void;
}

export default function ContractWizard({ onClose, onSuccess }: ContractWizardProps) {
  const [step, setStep] = useState(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  
  // Form State
  const [basicData, setBasicData] = useState({
    property_id: '',
    start_date: '',
    end_date: '',
    original_rent_amount: '',
    adjustment_method: 'IPC',
    adjustment_frequency: 'QUARTERLY',
    automation_mode: 'SEMIAUTOMATIC',
    fixed_percentage: '',
    notification_days_before: '30',
    c_type: 'HOUSING',
    c_destination: 'HABITATIONAL',
    currency: 'ARS',
    deposit_amount: '0',
  });

  const [participants, setParticipants] = useState<any[]>([]);

  const [terms, setTerms] = useState({
    allows_pets: false,
    allows_sublease: false,
    requires_inventory: false,
    requires_insurance: false,
    automatic_renewal: false,
    permitted_activity: '',
    notice_days: '30',
    early_termination_penalty: '',
    observations: ''
  });

  const [clauses, setClauses] = useState<any[]>([]);
  const [templateId, setTemplateId] = useState('');
  const [templates, setTemplates] = useState<any[]>([]);

  // Search State
  const [clients, setClients] = useState<Client[]>([]);
  const [properties, setProperties] = useState<any[]>([]);

  useEffect(() => {
    propertiesService.getAll(100, 0).then(data => setProperties(Array.isArray(data) ? data : data?.data || []));
    clientsService.getClients(100).then(res => setClients(res.data || []));
    fetchApi('/contracts/v2/contract-templates').then(data => setTemplates(data || []));
  }, []);

  const addParticipant = (role: string, is_main: boolean) => {
    setParticipants([...participants, {
      client_id: '',
      p_role: role,
      percentage: '100',
      is_main,
      guarantees: []
    }]);
  };

  const updateParticipant = (index: number, field: string, value: any) => {
    const newP = [...participants];
    newP[index][field] = value;
    setParticipants(newP);
  };

  const removeParticipant = (index: number) => {
    setParticipants(participants.filter((_, i) => i !== index));
  };

  const addGuarantee = (pIndex: number) => {
    const newP = [...participants];
    newP[pIndex].guarantees.push({
      guarantee_type: 'PAYSLIP',
      income_amount: '0',
      employer: '',
      guarantee_details: ''
    });
    setParticipants(newP);
  };

  const handleTemplateSelect = async (id: string) => {
    setTemplateId(id);
    if (!id) {
      setClauses([]);
      return;
    }
    const templateData = await fetchApi(`/contracts/v2/contract-templates/${id}`);
    if (templateData && templateData.clauses) {
      setClauses(templateData.clauses);
    }
  };

  const handleNext = () => {
    if (step === 1 && !basicData.property_id) {
      setError('Debes seleccionar una propiedad.');
      return;
    }
    setError('');
    setStep(step + 1);
  };

  const handleSubmit = async () => {
    try {
      setLoading(true);
      setError('');
      
      const payload = {
        ...basicData,
        original_rent_amount: Number(basicData.original_rent_amount),
        fixed_percentage: basicData.fixed_percentage ? Number(basicData.fixed_percentage) : null,
        notification_days_before: Number(basicData.notification_days_before),
        deposit_amount: Number(basicData.deposit_amount),
        commission_amount: 0,
        fees_amount: 0,
        participants: participants.map(p => ({
          ...p,
          percentage: Number(p.percentage),
          guarantees: p.guarantees.map((g: any) => ({
            ...g,
            income_amount: Number(g.income_amount)
          }))
        })),
        terms: {
          ...terms,
          notice_days: Number(terms.notice_days)
        },
        clauses: clauses.map((c, i) => ({
          ...c,
          display_order: i + 1
        })),
        template_id: templateId || null,
        status: 'DRAFT'
      };

      await fetchApi('/contracts/v2', {
        method: 'POST',
        body: JSON.stringify(payload)
      });
      
      onSuccess();
    } catch (err: any) {
      setError(err.message || 'Error al crear el contrato');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 overflow-y-auto">
      <div className="bg-zinc-900 border border-zinc-800 rounded-xl shadow-2xl w-full max-w-4xl flex flex-col max-h-[90vh]">
        <div className="flex justify-between items-center p-6 border-b border-zinc-800">
          <h2 className="text-xl font-semibold text-white">Nuevo Contrato V2</h2>
          <button onClick={onClose} className="text-zinc-400 hover:text-white">
            <X size={24} />
          </button>
        </div>

        <div className="p-6 overflow-y-auto flex-1">
          {error && <div className="mb-4 p-3 bg-red-500/20 text-red-400 rounded-lg">{error}</div>}
          
          <div className="flex mb-6 space-x-2">
            {[1, 2, 3].map(s => (
              <div key={s} className={`h-2 flex-1 rounded-full ${step >= s ? 'bg-cyan-500' : 'bg-zinc-800'}`} />
            ))}
          </div>

          {step === 1 && (
            <div className="space-y-4">
              <h3 className="text-lg text-white font-medium">Paso 1: Datos Básicos</h3>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Propiedad</label>
                  <select
                    className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                    value={basicData.property_id}
                    onChange={(e) => {
                      const propId = e.target.value;
                      setBasicData({...basicData, property_id: propId});
                      
                      const prop = properties.find(p => p.id === propId);
                      if (prop && prop.owners && prop.owners.length > 0) {
                        const newParticipants = [...participants.filter(p => p.p_role !== 'LANDLORD')];
                        prop.owners.forEach((o: any) => {
                          newParticipants.push({
                            client_id: o.client_id,
                            p_role: 'LANDLORD',
                            percentage: o.percentage.toString(),
                            is_main: true,
                            guarantees: []
                          });
                        });
                        setParticipants(newParticipants);
                      }
                    }}
                  >
                    <option value="">Seleccionar...</option>
                    {properties.map(p => (
                      <option key={p.id} value={p.id}>{p.title}</option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Monto Inicial (ARS)</label>
                  <Input 
                    type="number"
                    value={basicData.original_rent_amount}
                    onChange={(e) => setBasicData({...basicData, original_rent_amount: e.target.value})}
                  />
                </div>
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Fecha Inicio</label>
                  <Input 
                    type="date"
                    value={basicData.start_date}
                    onChange={(e) => setBasicData({...basicData, start_date: e.target.value})}
                  />
                </div>
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Fecha Fin</label>
                  <Input 
                    type="date"
                    value={basicData.end_date}
                    onChange={(e) => setBasicData({...basicData, end_date: e.target.value})}
                  />
                </div>
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Tipo</label>
                  <select
                    className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                    value={basicData.c_type}
                    onChange={(e) => setBasicData({...basicData, c_type: e.target.value})}
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
                    value={basicData.adjustment_method}
                    onChange={(e) => setBasicData({...basicData, adjustment_method: e.target.value})}
                  >
                    <option value="IPC">IPC</option>
                    <option value="ICL">ICL</option>
                    <option value="FIXED_PERCENTAGE">Porcentaje Fijo</option>
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
              
              {participants.map((p, i) => (
                <Card key={i} className="bg-zinc-800 border-zinc-700">
                  <CardContent className="p-4 flex gap-4 items-end">
                    <div className="flex-1">
                      <label className="block text-sm text-zinc-400 mb-1">Cliente ({p.p_role})</label>
                      <select
                        className="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                        value={p.client_id}
                        onChange={(e) => updateParticipant(i, 'client_id', e.target.value)}
                      >
                        <option value="">Seleccionar cliente...</option>
                        {clients.map(c => (
                          <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                        ))}
                      </select>
                    </div>
                    <div>
                      <label className="block text-sm text-zinc-400 mb-1">% Part.</label>
                      <Input type="number" value={p.percentage} onChange={(e) => updateParticipant(i, 'percentage', e.target.value)} className="w-24" />
                    </div>
                    <Button type="button" variant="ghost" onClick={() => removeParticipant(i)} className="text-red-400 hover:text-red-300">
                      <Trash2 size={20} />
                    </Button>
                  </CardContent>
                </Card>
              ))}
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
              
              <div className="text-sm text-zinc-400 mb-4">Los garantes son añadidos como clientes con sus respectivos respaldos.</div>

              {participants.map((p, i) => {
                if (p.p_role !== 'GUARANTOR') return null;
                return (
                  <Card key={i} className="bg-zinc-800 border-zinc-700 mb-4">
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
                          className="w-full bg-zinc-900 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                          value={p.client_id}
                          onChange={(e) => updateParticipant(i, 'client_id', e.target.value)}
                        >
                          <option value="">Seleccionar garante...</option>
                          {clients.map(c => (
                            <option key={c.id} value={c.id}>{c.first_name} {c.last_name}</option>
                          ))}
                        </select>
                      </div>

                      {p.guarantees.map((g: any, gIndex: number) => (
                        <div key={gIndex} className="grid grid-cols-3 gap-4 p-3 bg-zinc-900 rounded-lg border border-zinc-700">
                          <div>
                            <label className="block text-sm text-zinc-400 mb-1">Tipo de Respaldo</label>
                            <select
                              className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2 text-white"
                              value={g.guarantee_type}
                              onChange={(e) => {
                                const newP = [...participants];
                                newP[i].guarantees[gIndex].guarantee_type = e.target.value;
                                setParticipants(newP);
                              }}
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
                                value={g.employer}
                                onChange={(e) => {
                                  const newP = [...participants];
                                  newP[i].guarantees[gIndex].employer = e.target.value;
                                  setParticipants(newP);
                                }}
                              />
                            </div>
                            <Button type="button" variant="ghost" onClick={() => {
                              const newP = [...participants];
                              newP[i].guarantees.splice(gIndex, 1);
                              setParticipants(newP);
                            }} className="text-red-400">
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
                  <input type="checkbox" checked={terms.allows_pets} onChange={(e) => setTerms({...terms, allows_pets: e.target.checked})} />
                  <span>Permite Mascotas</span>
                </label>
                <label className="flex items-center space-x-2 text-white">
                  <input type="checkbox" checked={terms.allows_sublease} onChange={(e) => setTerms({...terms, allows_sublease: e.target.checked})} />
                  <span>Permite Subalquiler</span>
                </label>
                <label className="flex items-center space-x-2 text-white">
                  <input type="checkbox" checked={terms.requires_inventory} onChange={(e) => setTerms({...terms, requires_inventory: e.target.checked})} />
                  <span>Inventario Obligatorio</span>
                </label>
                <label className="flex items-center space-x-2 text-white">
                  <input type="checkbox" checked={terms.requires_insurance} onChange={(e) => setTerms({...terms, requires_insurance: e.target.checked})} />
                  <span>Seguro Obligatorio</span>
                </label>
                <label className="flex items-center space-x-2 text-white">
                  <input type="checkbox" checked={terms.automatic_renewal} onChange={(e) => setTerms({...terms, automatic_renewal: e.target.checked})} />
                  <span>Renovación Automática</span>
                </label>
              </div>

              <div className="grid grid-cols-2 gap-4 mt-4">
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Días Preaviso (Rescisión)</label>
                  <Input type="number" value={terms.notice_days} onChange={(e) => setTerms({...terms, notice_days: e.target.value})} />
                </div>
                <div>
                  <label className="block text-sm text-zinc-400 mb-1">Actividad Permitida</label>
                  <Input value={terms.permitted_activity} onChange={(e) => setTerms({...terms, permitted_activity: e.target.value})} />
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
                value={templateId}
                onChange={(e) => handleTemplateSelect(e.target.value)}
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
                <Button size="sm" onClick={() => setClauses([...clauses, { title: '', body: '', is_mandatory: false, is_editable: true, is_system: false }])}>
                  <Plus size={16} className="mr-1"/> Agregar Cláusula Libre
                </Button>
              </div>
              
              <div className="space-y-4">
                {clauses.map((c, idx) => (
                  <Card key={idx} className="bg-zinc-800 border-zinc-700">
                    <CardHeader className="p-3 pb-0 flex flex-row justify-between items-center">
                      <Input 
                        placeholder="Título de cláusula" 
                        value={c.title}
                        disabled={!c.is_editable}
                        onChange={(e) => {
                          const newC = [...clauses];
                          newC[idx].title = e.target.value;
                          setClauses(newC);
                        }}
                        className="font-medium bg-transparent border-none text-white focus-visible:ring-0 max-w-[300px]"
                      />
                      {!c.is_mandatory && (
                        <Button variant="ghost" size="sm" onClick={() => setClauses(clauses.filter((_, i) => i !== idx))} className="text-red-400 hover:text-red-300">
                          <Trash2 size={16} />
                        </Button>
                      )}
                    </CardHeader>
                    <CardContent className="p-3">
                      <textarea 
                        className="w-full bg-zinc-900 border border-zinc-700 rounded-lg p-3 text-white text-sm min-h-[100px]"
                        value={c.body}
                        disabled={!c.is_editable}
                        onChange={(e) => {
                          const newC = [...clauses];
                          newC[idx].body = e.target.value;
                          setClauses(newC);
                        }}
                        placeholder="Redacta la cláusula aquí..."
                      />
                      <div className="mt-2 flex space-x-4 text-xs text-zinc-500">
                        {c.is_mandatory && <span>🔒 Obligatoria (No se puede eliminar)</span>}
                        {!c.is_editable && <span>🔒 No editable</span>}
                      </div>
                    </CardContent>
                  </Card>
                ))}
                {clauses.length === 0 && <div className="text-zinc-500 text-sm">No hay cláusulas definidas.</div>}
              </div>
            </div>
          )}
        </div>

        <div className="p-6 border-t border-zinc-800 flex justify-between">
          <Button type="button" variant="ghost" onClick={step > 1 ? () => setStep(step - 1) : onClose}>
            {step > 1 ? 'Atrás' : 'Cancelar'}
          </Button>
          
          {step < 6 ? (
            <Button onClick={handleNext}>Siguiente</Button>
          ) : (
            <Button onClick={handleSubmit} disabled={loading}>
              {loading ? 'Creando...' : 'Finalizar y Crear Contrato'}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
