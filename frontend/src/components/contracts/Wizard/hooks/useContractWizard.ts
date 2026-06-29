import { useState, useEffect, useCallback, useRef } from 'react';
import { useForm, UseFormReturn } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { contractSchema, ContractFormValues } from '../schema';
import { propertiesService } from '@/services/properties';
import { DraftStorage } from '@/services/draft-storage';
import { logger } from '@/lib/logger';

export function useContractWizard(initialPropertyId?: string | null) {
  const [step, setStep] = useState(1);
  const [globalError, setGlobalError] = useState('');
  const [loadingInitial, setLoadingInitial] = useState(!!initialPropertyId);
  const [property, setProperty] = useState<any>(null);
  
  // Borrador
  const [draftAvailable, setDraftAvailable] = useState<boolean>(false);
  const [isRestoring, setIsRestoring] = useState(false);

  const methods = useForm<ContractFormValues>({
    resolver: zodResolver(contractSchema) as any,
    defaultValues: {
      property_id: initialPropertyId || '',
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

  const isSubmitting = methods.formState.isSubmitting;
  
  // Guardar correlationId en ref para no perderlo
  const correlationIdRef = useRef<string>('');

  useEffect(() => {
    // Generate correlation ID
    if (!correlationIdRef.current) {
      correlationIdRef.current = crypto.randomUUID();
      (window as any).__CORRELATION_ID__ = correlationIdRef.current;
      logger.info('Wizard', 'wizard_open', { correlation_id: correlationIdRef.current, property_id: initialPropertyId });
    }

    const tenant = localStorage.getItem('tenant_id') || 'unknown';
    const userRaw = localStorage.getItem('user');
    const user = userRaw ? JSON.parse(userRaw).id : 'unknown';

    // Chequear draft
    if (DraftStorage.exists(tenant, initialPropertyId || null, user)) {
      setDraftAvailable(true);
    } else {
      loadInitialProperty();
    }
    
    // Cleanup de drafts expirados de paso
    DraftStorage.clearExpired();
  }, []);

  const loadInitialProperty = useCallback(async () => {
    if (!initialPropertyId) return;
    try {
      setLoadingInitial(true);
      const p = await propertiesService.getById(initialPropertyId);
      setProperty(p);
      
      methods.setValue('property_id', p.id);
      if (p.currency) methods.setValue('currency', p.currency);
      if (p.price) methods.setValue('original_rent_amount', Number(p.price));
      if (p.type) methods.setValue('c_type', p.type);
      if (p.destination) methods.setValue('c_destination', p.destination);

      if (p.owners && p.owners.length > 0) {
        const ownersAsLandlords = p.owners.map((o: any) => ({
          client_id: o.client_id,
          p_role: 'LANDLORD',
          percentage: o.percentage,
          is_main: true,
          guarantees: []
        }));
        methods.setValue('participants', ownersAsLandlords);
      }
      logger.info('Wizard', 'wizard_hydrated', { property_id: initialPropertyId });
    } catch (err) {
      logger.error('Wizard', 'property_hydration_error', { err, property_id: initialPropertyId });
      setGlobalError("No se pudo cargar la información de la propiedad preseleccionada.");
    } finally {
      setLoadingInitial(false);
    }
  }, [initialPropertyId, methods]);

  const handleRestoreDraft = useCallback(() => {
    const tenant = localStorage.getItem('tenant_id') || 'unknown';
    const userRaw = localStorage.getItem('user');
    const user = userRaw ? JSON.parse(userRaw).id : 'unknown';
    
    const draft = DraftStorage.load(tenant, initialPropertyId || null, user);
    if (draft && draft.payload) {
      methods.reset(draft.payload);
      logger.info('Wizard', 'draft_restored');
    }
    setDraftAvailable(false);
    // Cargar info de la property para solo lectura (PropertySummary) si es property centric
    if (initialPropertyId) {
      propertiesService.getById(initialPropertyId).then(setProperty).catch(console.error);
    }
  }, [initialPropertyId, methods]);

  const handleDiscardDraft = useCallback(() => {
    const tenant = localStorage.getItem('tenant_id') || 'unknown';
    const userRaw = localStorage.getItem('user');
    const user = userRaw ? JSON.parse(userRaw).id : 'unknown';
    
    DraftStorage.remove(tenant, initialPropertyId || null, user);
    setDraftAvailable(false);
    logger.info('Wizard', 'draft_discarded');
    loadInitialProperty();
  }, [initialPropertyId, loadInitialProperty]);

  // Auto-Save con debounce
  useEffect(() => {
    if (draftAvailable || isRestoring || loadingInitial || isSubmitting) return;

    const tenant = localStorage.getItem('tenant_id') || 'unknown';
    const userRaw = localStorage.getItem('user');
    const user = userRaw ? JSON.parse(userRaw).id : 'unknown';
    const cid = correlationIdRef.current;

    const subscription = methods.watch((value) => {
      const handler = setTimeout(() => {
        // Double check flag isSubmitting, as it might have changed
        if (!methods.formState.isSubmitting) {
          DraftStorage.save(tenant, initialPropertyId || null, user, value, cid);
          logger.debug('Wizard', 'draft_saved', { step });
        }
      }, 2000);
      
      return () => clearTimeout(handler);
    });

    return () => subscription.unsubscribe();
  }, [methods, draftAvailable, isRestoring, loadingInitial, isSubmitting, initialPropertyId, step]);

  const handleNext = useCallback(async () => {
    setGlobalError('');
    let fieldsToValidate: (keyof ContractFormValues)[] = [];
    
    if (step === 1) fieldsToValidate = ['property_id', 'start_date', 'end_date', 'original_rent_amount', 'adjustment_method'];
    if (step === 2 || step === 3) fieldsToValidate = ['participants'];
    
    const isValid = await methods.trigger(fieldsToValidate as any);
    
    if (!isValid) {
      const errs = methods.formState.errors.participants;
      const errorDetail = errs ? JSON.stringify(errs) : 'Desconocido';
      setGlobalError(`Error de validación en participantes: ${errorDetail}`);
      return;
    }

    if (step === 2 || step === 3) {
      const parts = methods.getValues('participants');
      const hasLandlord = parts.some(p => p.p_role === 'LANDLORD' && p.is_main);
      const hasTenant = parts.some(p => p.p_role === 'TENANT' && p.is_main);
      if (!hasLandlord || !hasTenant) {
         setGlobalError('Debes incluir al menos un locador principal y un locatario principal.');
         return;
      }
    }

    setStep(prev => prev + 1);
    logger.info('Wizard', 'wizard_step_change', { from: step, to: step + 1 });
  }, [step, methods]);

  const handleBack = useCallback(() => {
    if (step > 1) {
      setStep(prev => prev - 1);
      logger.info('Wizard', 'wizard_step_change', { from: step, to: step - 1 });
    }
  }, [step]);

  const clearDraft = useCallback(() => {
    const tenant = localStorage.getItem('tenant_id') || 'unknown';
    const userRaw = localStorage.getItem('user');
    const user = userRaw ? JSON.parse(userRaw).id : 'unknown';
    DraftStorage.remove(tenant, initialPropertyId || null, user);
  }, [initialPropertyId]);

  const isPropertyCentric = !!initialPropertyId;

  return {
    methods,
    step,
    setStep,
    globalError,
    setGlobalError,
    handleNext,
    handleBack,
    loadingInitial,
    isPropertyCentric,
    property,
    draftAvailable,
    handleRestoreDraft,
    handleDiscardDraft,
    clearDraft
  };
}
