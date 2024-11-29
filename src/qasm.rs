use std::collections::HashSet;
use num_bigint::BigUint;
use num_complex::Complex;
use qsc::{interpret::Value, Backend};

pub(crate) struct Qasm2Backend {
    code: Vec<String>,
    errors: Vec<String>,
    qubits: HashSet<usize>,
    next_qubit_id: usize,
    cbit_counter: usize,
}

impl Qasm2Backend {
    pub fn new() -> Self {
        Qasm2Backend {
            code: Vec::new(),
            qubits: HashSet::new(),
            next_qubit_id: 0,
            cbit_counter: 0,
            errors: Vec::new(),
        }
    }

    pub fn get_qasm(&self) -> Result<String, Vec<String>> {
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        let mut qasm = String::new();
        qasm.push_str("OPENQASM 2.0;\n");
        qasm.push_str("include \"qelib1.inc\";\n");
        let n_qubits = self.next_qubit_id;
        qasm.push_str(&format!("qreg q[{}];\n", n_qubits));
        if self.cbit_counter > 0 {
            qasm.push_str(&format!("creg c[{}];\n", self.cbit_counter));
        }
        for line in &self.code {
            qasm.push_str(line);
            qasm.push('\n');
        }
        Ok(qasm)
    }
}

impl Backend for Qasm2Backend {
    type ResultType = bool;

    fn qubit_allocate(&mut self) -> usize {
        let q = self.next_qubit_id;
        self.next_qubit_id += 1;
        self.qubits.insert(q);
        q
    }

    fn qubit_release(&mut self, q: usize) -> bool {
        self.qubits.remove(&q);
        true
    }

    fn h(&mut self, q: usize) {
        self.code.push(format!("h q[{}];", q));
    }

    fn x(&mut self, q: usize) {
        self.code.push(format!("x q[{}];", q));
    }

    fn y(&mut self, q: usize) {
        self.code.push(format!("y q[{}];", q));
    }

    fn z(&mut self, q: usize) {
        self.code.push(format!("z q[{}];", q));
    }

    fn s(&mut self, q: usize) {
        self.code.push(format!("s q[{}];", q));
    }

    fn t(&mut self, q: usize) {
        self.code.push(format!("t q[{}];", q));
    }

    fn sadj(&mut self, q: usize) {
        self.code.push(format!("sdg q[{}];", q));
    }

    fn tadj(&mut self, q: usize) {
        self.code.push(format!("tdg q[{}];", q));
    }

    fn rx(&mut self, theta: f64, q: usize) {
        self.code.push(format!("rx({}) q[{}];", theta, q));
    }

    fn ry(&mut self, theta: f64, q: usize) {
        self.code.push(format!("ry({}) q[{}];", theta, q));
    }

    fn rz(&mut self, theta: f64, q: usize) {
        self.code.push(format!("rz({}) q[{}];", theta, q));
    }

    fn cx(&mut self, ctl: usize, q: usize) {
        self.code.push(format!("cx q[{}], q[{}];", ctl, q));
    }

    fn cz(&mut self, ctl: usize, q: usize) {
        self.code.push(format!("cz q[{}], q[{}];", ctl, q));
    }

    fn swap(&mut self, q0: usize, q1: usize) {
        self.code.push(format!("swap q[{}], q[{}];", q0, q1));
    }

    fn ccx(&mut self, ctl0: usize, ctl1: usize, q: usize) {
        self.code
            .push(format!("ccx q[{}], q[{}], q[{}];", ctl0, ctl1, q));
    }

    fn m(&mut self, q: usize) -> Self::ResultType {
        let c = self.cbit_counter;
        self.cbit_counter += 1;
        self.code.push(format!("measure q[{}] -> c[{}];", q, c));
        false
    }

    fn mresetz(&mut self, _q: usize) -> Self::ResultType {
        self.errors.push("Reset not supported in QASM 2.0".to_string());
        false
    }

    fn reset(&mut self, _q: usize) {
        self.errors.push("Reset not supported in QASM 2.0".to_string());
    }

    fn rxx(&mut self, theta: f64, q0: usize, q1: usize) {
        // todo: check if rxx is supported in qasm 2.0, probably not
        self.code
            .push(format!("rxx({}) q[{}], q[{}];", theta, q0, q1));
    }

    fn ryy(&mut self, theta: f64, q0: usize, q1: usize) {
        // todo: check if rxx is supported in qasm 2.0, probably not
        self.code
            .push(format!("ryy({}) q[{}], q[{}];", theta, q0, q1));
    }

    fn rzz(&mut self, theta: f64, q0: usize, q1: usize) {
        // todo: check if rxx is supported in qasm 2.0, probably not
        self.code
            .push(format!("rzz({}) q[{}], q[{}];", theta, q0, q1));
    }

    fn cy(&mut self, ctl: usize, q: usize) {
        self.code.push(format!("cy q[{}], q[{}];", ctl, q));
    }

    fn qubit_swap_id(&mut self, q0: usize, q1: usize) {
        self.swap(q0, q1);
    }

    fn capture_quantum_state(&mut self) -> (Vec<(BigUint, Complex<f64>)>, usize) {
        // not supported in qasm 2.0
        // however we can treat is as no-op
        (Vec::new(), 0)
    }

    fn qubit_is_zero(&mut self, _q: usize) -> bool {
        self.errors.push("Qubit is_zero not supported in QASM 2.0".to_string());
        false
    }

    fn custom_intrinsic(
        &mut self,
        name: &str,
        _arg: Value,
    ) -> Option<Result<Value, String>> {
        self.errors.push(format!(
            "Custom intrinsic '{}' not supported in QASM 2.0",
            name
        ));
        None
    }

    fn set_seed(&mut self, _seed: Option<u64>) {
        self.errors.push("Set seed not supported in QASM 2.0".to_string());
    }
}

