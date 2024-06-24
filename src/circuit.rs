#[derive(Clone, Default, Debug, PartialEq)]
pub struct Circuit {
    pub operations: Vec<Operation>,
    pub qubits: Vec<Qubit>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub gate: String,
    pub display_args: Option<String>,
    pub is_controlled: bool,
    pub is_adjoint: bool,
    pub is_measurement: bool,
    pub controls: Vec<Register>,
    pub targets: Vec<Register>,
    pub children: Vec<Operation>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Register {
    pub q_id: u64,
    pub register_type: u64,
    pub c_id: Option<u64>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Qubit {
    pub id: u64,
    pub num_children: u64,
}

impl From<qsc::circuit::Circuit> for Circuit {
    fn from(circuit: qsc::circuit::Circuit) -> Self {
        Circuit {
            operations: circuit.operations.into_iter().map(Operation::from).collect(),
            qubits: circuit.qubits.into_iter().map(|q| Qubit {
                id: q.id as u64,
                num_children: q.num_children as u64,
            }).collect(),
        }
    }
}

impl From<qsc::circuit::Operation> for Operation {
    fn from(op: qsc::circuit::Operation) -> Self {
        Operation {
            gate: op.gate,
            display_args: op.display_args,
            is_controlled: op.is_controlled,
            is_adjoint: op.is_adjoint,
            is_measurement: op.is_measurement,
            controls: op.controls.into_iter().map(|c| Register {
                q_id: c.q_id as u64,
                register_type: c.r#type as u64,
                c_id: c.c_id.map(|id| id as u64),
            }).collect(),
            targets: op.targets.into_iter().map(|t| Register {
                q_id: t.q_id as u64,
                register_type: t.r#type as u64,
                c_id: t.c_id.map(|id| id as u64),
            }).collect(),
            children: op.children.into_iter().map(Operation::from).collect(),
        }
    }
}