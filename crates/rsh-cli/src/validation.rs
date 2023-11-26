use rsh_parser::parse;
use rsh_protocol::{
    engine::{EngineState, StateWorkingSet},
    ParseError,
};
use reedline::{ValidationResult, Validator};
use std::sync::Arc;

pub struct RshValidator {
    pub engine_state: Arc<EngineState>,
}

impl Validator for RshValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        let mut working_set = StateWorkingSet::new(&self.engine_state);
        parse(&mut working_set, None, line.as_bytes(), false);

        if matches!(
            working_set.parse_errors.first(),
            Some(ParseError::UnexpectedEof(..))
        ) {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}
