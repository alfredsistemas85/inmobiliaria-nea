import { z } from 'zod';

export const guaranteeSchema = z.object({
  guarantee_type: z.string(),
  income_amount: z.coerce.number().min(0),
  employer: z.string().optional(),
  guarantee_details: z.string().optional()
});

export const participantSchema = z.object({
  client_id: z.string().min(1, "Selecciona un cliente").uuid("ID inválido"),
  p_role: z.string(),
  percentage: z.coerce.number().min(0, "Mín. 0").max(100, "Máx. 100"),
  is_main: z.boolean(),
  guarantees: z.array(guaranteeSchema).default([])
});

export const clauseSchema = z.object({
  title: z.string().default(""),
  body: z.string().default(""),
  is_mandatory: z.boolean().default(false),
  is_editable: z.boolean().default(true),
  is_system: z.boolean().default(false)
});

export const contractSchema = z.object({
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

export type ContractFormValues = z.infer<typeof contractSchema>;
