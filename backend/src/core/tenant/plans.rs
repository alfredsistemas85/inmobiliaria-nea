use crate::models::subscription::PlanType;

pub struct PlanLimits {
    pub max_users: Option<u32>,
    pub max_properties: Option<u32>,
}

pub fn get_plan_limits(plan: &PlanType) -> PlanLimits {
    match plan {
        PlanType::Basic => PlanLimits {
            max_users: Some(5),
            max_properties: Some(500),
        },
        PlanType::Pro => PlanLimits {
            max_users: None,
            max_properties: None,
        },
    }
}
