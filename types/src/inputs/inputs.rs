use crate::{FunctionInput, InputValue, ProgramInputs, ProgramState};
use leo_inputs::{files::File, sections::header::Header, InputParserError};

#[derive(Clone)]
pub struct Inputs {
    program_inputs: Vec<Option<InputValue>>,
    inputs: ProgramInputs,
    state: ProgramState,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            program_inputs: vec![],
            inputs: ProgramInputs::new(),
            state: ProgramState::new(),
        }
    }

    pub fn get_inputs(&self) -> Vec<Option<InputValue>> {
        self.program_inputs.clone()
    }

    pub fn set_inputs(&mut self, inputs: Vec<Option<InputValue>>) {
        self.program_inputs = inputs;
    }

    pub fn set_inputs_size(&mut self, size: usize) {
        self.program_inputs = vec![None; size];
    }

    pub fn from_inputs_file(file: File, expected_inputs: Vec<FunctionInput>) -> Result<Self, InputParserError> {
        let mut program_inputs = vec![];

        for section in file.sections.into_iter() {
            match section.header {
                Header::Main(_main) => {
                    for input in &expected_inputs {
                        // find input with matching name
                        let matched_input = section.assignments.clone().into_iter().find(|assignment| {
                            // name match
                            assignment.parameter.variable.value.eq(&input.identifier.name)
                                // type match
                                && assignment.parameter.type_.to_string().eq(&input._type.to_string())
                        });

                        match matched_input {
                            Some(assignment) => {
                                let value =
                                    InputValue::from_expression(assignment.parameter.type_, assignment.expression)?;

                                // push value to vector
                                program_inputs.push(Some(value));
                            }
                            None => return Err(InputParserError::InputNotFound(input.to_string())),
                        }
                    }
                }
                _ => unimplemented!("section not impl"),
            }
        }

        Ok(Self {
            program_inputs,
            inputs: ProgramInputs::new(),
            state: ProgramState::new(),
        })
    }
}
