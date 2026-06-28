use uuid::Uuid;

#[derive(Debug)]
pub struct InstallmentDto {
    pub id: Uuid,
    pub amount: f64,
}
