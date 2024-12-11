use std::sync::Arc;

use qsc::interpret::{self, GenericReceiver, Interpreter};
use resource_estimator::{estimate_entry, estimate_expr};
use thiserror::Error;
use num_bigint::BigUint;
use num_complex::Complex64;
use qsc::interpret::output::Receiver;
use qsc::interpret::output;
use qsc::{format_state_id, LanguageFeatures, PackageType, PauliNoise, SourceMap, SparseSim, TargetCapabilityFlags};

use crate::circuit::Circuit;
use crate::qasm::{Qasm2Backend, QasmGenerationOptions};

pub struct ExecutionOptions {
    pub shots: u32,
    pub noise: PauliDistribution,
}

pub struct PauliDistribution {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PauliDistribution {
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, QsError> {
        if x < 0.0 || y < 0.0 || z < 0.0 || x + y + z > 1.0 {
            return Err(QsError::ErrorMessage { error_text: "Invalid Pauli distribution: values must be non-negative and sum to <= 1.0".to_string() });
        }
        Ok(Self { x, y, z })
    }
}


impl ExecutionOptions {
    pub fn new(shots: u32, noise: PauliDistribution) -> Self {
        Self { shots, noise }
    }

    pub fn from_shots(shots: u32) -> Self {
        Self {
            shots,
            ..Default::default()
        }
    }

    pub fn from_noise(noise: PauliDistribution) -> Self {
        Self {
            noise,
            ..Default::default()
        }
    }
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            shots: 1,
            noise: PauliDistribution::new(0.0, 0.0, 0.0).unwrap(),
        }
    }
}

pub fn circuit(source: &str) -> Result<Circuit, QsError> {
    let mut interpreter = create_interpreter(Some(source), PackageType::Exe, TargetCapabilityFlags::all())?;
    let mut rec = ExecutionState::default();
    let _ = interpreter.eval_entry(&mut rec)?;
    let result = interpreter.get_circuit();
    return Ok(result.into());
}

pub fn run_qs(source: &str) -> Result<ExecutionState, QsError> {
    let mut interpreter = create_interpreter(Some(source), PackageType::Exe, TargetCapabilityFlags::all())?;
    let mut rec = ExecutionState::default();
    let result = interpreter.eval_entry(&mut rec)?;
    rec.set_result(result.to_string());
    return Ok(rec);
}

pub fn run_qs_with_options(source: &str, options: Arc<ExecutionOptions>) -> Result<Vec<ExecutionState>, QsError> {
    let mut results: Vec<ExecutionState> = Vec::new();
    let mut interpreter = create_interpreter(Some(source), PackageType::Exe, TargetCapabilityFlags::all())?;

    let mut sim = if options.noise.x == 0.0 && options.noise.y == 0.0 && options.noise.z == 0.0 {
        SparseSim::new() //default
    } else {
        let noise = PauliNoise::from_probabilities(options.noise.x, options.noise.y, options.noise.z).map_err(|error| QsError::ErrorMessage { error_text: error })?;
        SparseSim::new_with_noise(&noise)
    };

    let shots = options.shots;
    for _ in 0..shots {
        let mut rec = ExecutionState::default();
        let result = interpreter.eval_entry_with_sim(&mut sim, &mut rec)?;
        rec.set_result(result.to_string());
        results.push(rec)
    }

    return Ok(results);
}

pub fn qir(expression: &str) -> Result<String, QsError> {
    let mut interpreter = create_interpreter(None, PackageType::Lib, TargetCapabilityFlags::empty())?;
    let result = interpreter.qirgen(expression)?;
    return Ok(result);
}

pub fn estimate(source: &str, job_params: Option<String>) -> Result<String, QsError> {
    let mut interpreter = create_interpreter(Some(source), PackageType::Exe, TargetCapabilityFlags::empty())?;
    let params = job_params.as_deref().unwrap_or("[{}]");
    let result = estimate_entry(&mut interpreter, params)?;
    return Ok(result);
}

pub fn estimate_expression(expression: &str, job_params: Option<String>) -> Result<String, QsError> {
    let mut interpreter = create_interpreter(None, PackageType::Lib, TargetCapabilityFlags::empty())?;
    let params = job_params.as_deref().unwrap_or("[{}]");
    let result = estimate_expr(&mut interpreter, expression, params)?;
    return Ok(result);
}

pub fn qasm2(source: &str, generation_options: QasmGenerationOptions) -> Result<String, QsError> {
    let mut stdout = vec![];
    let mut out = GenericReceiver::new(&mut stdout);
    let mut backend = Qasm2Backend::new(generation_options);

    let mut interpreter = create_interpreter(Some(source), PackageType::Exe, TargetCapabilityFlags::empty())?;
    let _ = interpreter.eval_entry_with_sim(&mut backend, &mut out)?;

    let qasm = backend.get_qasm().map_err(|errors| QsError::ErrorMessage { error_text: errors.join(", ") })?;
    Ok(qasm)
}

pub fn qasm2_expression(expression: &str, generation_options: QasmGenerationOptions) -> Result<String, QsError> {
    let mut stdout = vec![];
    let mut out = GenericReceiver::new(&mut stdout);
    let mut backend = Qasm2Backend::new(generation_options);

    let mut interpreter = create_interpreter(None, PackageType::Lib, TargetCapabilityFlags::empty())?;
    let _ = interpreter.run_with_sim(&mut backend, &mut out, Some(expression))?;

    let qasm = backend.get_qasm().map_err(|errors| QsError::ErrorMessage { error_text: errors.join(", ") })?;
    Ok(qasm)
}

fn create_interpreter(source: Option<&str>, package_type: PackageType, target_capability_flags: TargetCapabilityFlags) -> Result<Interpreter, QsError> {
    let source_map = match source {
        Some(source) => SourceMap::new(vec![("temp.qs".into(), source.into())], None),
        None => SourceMap::default(),
    };

    let (std_id, store) = qsc::compile::package_store_with_stdlib(TargetCapabilityFlags::all());
    let interpreter = match Interpreter::new(
        source_map,
        package_type,
        target_capability_flags,
        LanguageFeatures::default(),
        store,
        &[(std_id, None)],
    ) {
        Ok(interpreter) => interpreter,
        Err(errors) => {
            return Err(errors.into());
        }
    };

    return Ok(interpreter);
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

    fn matrix(&mut self, _matrix: Vec<Vec<Complex64>>) -> Result<(), output::Error> {
        // todo
        Ok(())
    }
}

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
                    let QsError::ErrorMessage { error_text } = qs_error;
                    error_message.push_str(&error_text);
                },
                resource_estimator::Error::Estimation(estimates_error) => {
                    // Handle `estimates::Error` similarly, if applicable
                    error_message.push_str(&format!("Estimation error: {:?}", estimates_error));
                }
            }
        }

        // ensure that the leading ", " is removed if it's the start of the error message
        QsError::ErrorMessage { error_text: error_message.trim_start_matches(", ").to_string() }
    }
}