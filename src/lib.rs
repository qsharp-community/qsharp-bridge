uniffi::include_scaffolding!("qsharp-bridge");

use qsc::interpret::{Interpreter, self};
use resource_estimator::{estimate_entry, estimate_expr};
use thiserror::Error;
use num_bigint::BigUint;
use num_complex::Complex64;
use qsc::interpret::output::Receiver;
use qsc::interpret::output;
use qsc::{format_state_id, LanguageFeatures, PackageType, TargetCapabilityFlags, SourceMap};

#[derive(Error, Debug)]
pub enum QsError {
    #[error("Error with message: `{error_text}`")]
    ErrorMessage { error_text: String }
}

impl From<Vec<interpret::Error>> for QsError {
    fn from(errors: Vec<interpret::Error>) -> Self {
        let mut error_message = String::new();

        for error in errors {
            if let Some(stack_trace) = error.stack_trace() {
                error_message.push_str(&format!("Stack trace: {}", stack_trace));
            }

            error_message.push_str(&format!(", error: {:?}", error));
        }

        QsError::ErrorMessage { error_text: error_message }
    }
}

impl From<Vec<resource_estimator::Error>> for QsError {
    fn from(errors: Vec<resource_estimator::Error>) -> Self {
        let mut error_message = String::new();

        for error in errors {
            match error {
                resource_estimator::Error::Interpreter(interpret_error) => {
                    let qs_error: QsError = vec![interpret_error].into();
                    if let QsError::ErrorMessage { error_text } = qs_error {
                        error_message.push_str(&error_text);
                    }
                },
                resource_estimator::Error::Estimation(estimates_error) => {
                    // Handle `estimates::Error` similarly, if applicable
                    error_message.push_str(&format!("Estimation error: {:?}", estimates_error));
                }
            }
        }

        // Ensure that the leading ", " is removed if it's the start of the error message
        QsError::ErrorMessage { error_text: error_message.trim_start_matches(", ").to_string() }
    }
}


pub fn run_qs(source: &str) -> Result<ExecutionState, QsError> {
    let source_map = SourceMap::new(vec![("temp.qs".into(), source.into())], None);
    let mut interpreter = match Interpreter::new(
        true,
        source_map,
        PackageType::Exe,
        TargetCapabilityFlags::all(),
        LanguageFeatures::default()
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    let mut rec = ExecutionState::default();
    let result = interpreter.eval_entry(&mut rec)?;
    rec.set_result(result.to_string());
    return Ok(rec);
}

pub fn run_qs_shots(source: &str, shots: u32) -> Result<Vec<ExecutionState>, QsError> {
    let mut results: Vec<ExecutionState> = Vec::new();

    let source_map = SourceMap::new(vec![("temp.qs".into(), source.into())], None);

    let mut interpreter = match Interpreter::new(
        true,
        source_map,
        PackageType::Exe,
        TargetCapabilityFlags::all(),
        LanguageFeatures::default()
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    for _ in 0..shots {
        let mut rec = ExecutionState::default();
        let result = interpreter.eval_entry(&mut rec)?;
        rec.set_result(result.to_string());
        results.push(rec)
    }

    return Ok(results);
}

pub fn qir(expression: &str) -> Result<String, QsError> {
    let mut interpreter = match Interpreter::new(
        true,
        SourceMap::default(),
        PackageType::Lib,
        TargetCapabilityFlags::empty(),
        LanguageFeatures::default()
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    let result = interpreter.qirgen(expression)?;
    return Ok(result);
}

pub fn estimate(source: &str, job_params: Option<String>) -> Result<String, QsError> {
    let source_map = SourceMap::new(vec![("temp.qs".into(), source.into())], None);
    let mut interpreter = match Interpreter::new(
        true,
        source_map,
        PackageType::Exe,
        TargetCapabilityFlags::empty(),
        LanguageFeatures::default()
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    let params = job_params.as_deref().unwrap_or("[{}]");
    let result = estimate_entry(&mut interpreter, params)?;
    return Ok(result);
}

pub fn estimate_expression(expression: &str, job_params: Option<String>) -> Result<String, QsError> {
    let mut interpreter = match Interpreter::new(
        true,
        SourceMap::default(),
        PackageType::Lib,
        TargetCapabilityFlags::empty(),
        LanguageFeatures::default()
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    let params = job_params.as_deref().unwrap_or("[{}]");
    let result = estimate_expr(&mut interpreter, expression, params)?;
    return Ok(result);
}


pub struct QubitState {
    pub id: String,
    pub amplitude_real: f64,
    pub amplitude_imaginary: f64,
}

pub struct ExecutionState {
    pub states: Vec<QubitState>,
    pub qubit_count: u64,
    pub messages: Vec<String>,
    pub result: Option<String>,
}

impl ExecutionState {
    fn set_result(&mut self, result: String) {
        self.result = Some(result);
    }
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            states: Vec::new(),
            qubit_count: 0,
            messages: Vec::new(),
            result: None,
        }
    }
}

impl Receiver for ExecutionState {
    fn state(
        &mut self,
        states: Vec<(BigUint, Complex64)>,
        qubit_count: usize,
    ) -> Result<(), output::Error> {
        self.qubit_count = qubit_count as u64;
        self.states = states.iter().map(|(qubit, amplitude)| {
            QubitState {
                id: format_state_id(&qubit, qubit_count),
                amplitude_real: amplitude.re,
                amplitude_imaginary: amplitude.im,
            }
        }).collect();

        Ok(())
    }

    fn message(&mut self, msg: &str) -> Result<(), output::Error> {
        self.messages.push(msg.to_string());
        Ok(())
    }
}