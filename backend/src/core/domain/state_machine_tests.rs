#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(ContractStateMachine::validate_transition(&ContractStatus::Draft, &ContractStatus::PendingSignature).is_ok());
        assert!(ContractStateMachine::validate_transition(&ContractStatus::PendingSignature, &ContractStatus::Active).is_ok());
        assert!(ContractStateMachine::validate_transition(&ContractStatus::Active, &ContractStatus::Finished).is_ok());
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(ContractStateMachine::validate_transition(&ContractStatus::Draft, &ContractStatus::Active).is_err());
        assert!(ContractStateMachine::validate_transition(&ContractStatus::Active, &ContractStatus::Draft).is_err());
        assert!(ContractStateMachine::validate_transition(&ContractStatus::Annulled, &ContractStatus::Active).is_err());
    }
}
