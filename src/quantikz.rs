use qsc_circuit::{Circuit, Operation};
use std::collections::{HashMap, HashSet};

type RegisterMap = HashMap<(usize, Option<usize>), usize>;

#[derive(Clone)]
struct Row {
    label: Option<String>,
    is_classical: bool,
}

pub fn circuit_to_quantikz(c: &Circuit) -> String {
    let (mut rows, register_to_row) = build_rows(c);
    let col_count = c.component_grid.len();
    let mut table = initialize_table(rows.len(), col_count, &rows);

    populate_table(c, &register_to_row, &mut table, &mut rows);

    render_latex(&rows, &table)
}

fn build_rows(c: &Circuit) -> (Vec<Row>, RegisterMap) {
    let mut rows: Vec<Row> = Vec::new();
    let mut register_to_row = HashMap::new();
    let qubits_with_gap_row_below = identify_gaps(c);

    for q in &c.qubits {
        let label = format!("\\lstick{{\\ket{{0}}_{{{}}}}}", q.id);
        register_to_row.insert((q.id, None), rows.len());
        rows.push(Row {
            label: Some(label),
            is_classical: false,
        });

        let extra_rows = if qubits_with_gap_row_below.contains(&q.id) {
            std::cmp::max(1, q.num_results)
        } else {
            q.num_results
        };

        for i in 0..extra_rows {
            register_to_row.insert((q.id, Some(i)), rows.len());
            rows.push(Row {
                label: None,
                is_classical: true,
            });
        }
    }
    (rows, register_to_row)
}

fn identify_gaps(c: &Circuit) -> HashSet<usize> {
    let mut gaps = HashSet::new();
    for col in &c.component_grid {
        for op in &col.components {
            let targets = match op {
                Operation::Measurement(m) => &m.qubits,
                Operation::Unitary(u) => &u.targets,
                Operation::Ket(k) => &k.targets,
            };
            for target in targets {
                let q = target.qubit;
                if gaps.contains(&q) {
                    continue;
                }
                let next_q = q + 1;
                if targets.iter().any(|t| t.qubit == next_q) {
                    gaps.insert(q);
                }
            }
        }
    }
    gaps
}

fn initialize_table(row_count: usize, col_count: usize, rows: &[Row]) -> Vec<Vec<String>> {
    let mut table = vec![vec![String::new(); col_count + 1]; row_count];
    for (r_idx, row) in rows.iter().enumerate() {
        for c_idx in 0..=col_count {
            table[r_idx][c_idx] = if row.is_classical {
                String::from("\\cw")
            } else {
                String::from("\\qw")
            };
        }
    }
    table
}

fn populate_table(
    c: &Circuit,
    register_to_row: &RegisterMap,
    table: &mut [Vec<String>],
    rows: &mut [Row],
) {
    for (col_index, col) in c.component_grid.iter().enumerate() {
        let table_col = col_index;
        for op in &col.components {
            let targets = get_rows_for_operation(op, register_to_row, true);
            let controls = get_rows_for_operation(op, register_to_row, false);

            process_operation(op, table_col, &targets, &controls, table, rows);
        }
    }
}

fn process_operation(
    op: &Operation,
    col: usize,
    targets: &[usize],
    controls: &[usize],
    table: &mut [Vec<String>],
    rows: &mut [Row],
) {
    match op {
        Operation::Unitary(u) => {
            process_unitary(
                &u.gate,
                &op.args(),
                u.is_adjoint,
                col,
                targets,
                controls,
                table,
            );
        }
        Operation::Measurement(_) => {
            for &t in targets {
                table[t][col] = String::from("\\meter{}");
                rows[t].is_classical = true;
                for next_c in (col + 1)..table[t].len() {
                    table[t][next_c] = String::from("\\cw");
                }
            }
        }
        Operation::Ket(_) => {
            for &t in targets {
                table[t][col] = String::from("\\gate{\\ket{0}}");
                rows[t].is_classical = false;
                for next_c in (col + 1)..table[t].len() {
                    table[t][next_c] = String::from("\\qw");
                }
            }
        }
    }
}

fn process_unitary(
    name: &str,
    args: &[String],
    is_adjoint: bool,
    col: usize,
    targets: &[usize],
    controls: &[usize],
    table: &mut [Vec<String>],
) {
    let simple_name = name.split('.').last().unwrap_or(name);
    let is_swap = simple_name.eq_ignore_ascii_case("swap");

    if is_swap && targets.len() == 2 {
        let t1 = targets[0];
        let t2 = targets[1];
        let offset = t2 as isize - t1 as isize;
        table[t1][col] = format!("\\swap{{{}}}", offset);
        table[t2][col] = String::from("\\targX{}");

        for &ctrl in controls {
            let offset = t1 as isize - ctrl as isize;
            table[ctrl][col] = format!("\\ctrl{{{}}}", offset);
        }
        return;
    }

    if (simple_name == "X" || simple_name == "CNOT") && !controls.is_empty() {
        for &t in targets {
            table[t][col] = String::from("\\targ{}");
        }
    } else if simple_name == "Z" && !controls.is_empty() {
        for &t in targets {
            let ctrl_ref = controls[0];
            let diff = t as isize - ctrl_ref as isize;
            table[t][col] = format!("\\ctrl{{{}}}", -diff);
        }
    } else {
        let label = operation_label(simple_name, args, is_adjoint);
        if targets.len() > 1 {
            let first_row = targets[0];
            table[first_row][col] = format!("\\gate[wires={}]{{{}}}", targets.len(), label);
        } else if targets.len() == 1 {
            let r = targets[0];
            table[r][col] = format!("\\gate{{{}}}", label);
        }
    }

    if !controls.is_empty() {
        for &ctrl in controls {
            let target = targets.get(0).copied().unwrap_or(ctrl);
            let offset = target as isize - ctrl as isize;
            table[ctrl][col] = format!("\\ctrl{{{}}}", offset);
        }
    }
}

fn render_latex(rows: &[Row], table: &[Vec<String>]) -> String {
    let mut out = String::new();
    out.push_str("\\begin{quantikz}\n");
    for (row_idx, row) in rows.iter().enumerate() {
        if let Some(l) = &row.label {
            out.push_str(l);
        }
        out.push_str(" & ");
        out.push_str(&table[row_idx].join(" & "));
        out.push_str(" \\\\\n");
    }
    out.push_str("\\end{quantikz}\n");
    out
}

fn operation_label(name: &str, args: &[String], is_adjoint: bool) -> String {
    let mut lbl = match name {
        "Rx" => "R_x".to_string(),
        "Ry" => "R_y".to_string(),
        "Rz" => "R_z".to_string(),
        "R1" => "R_1".to_string(),
        n => n.to_string(),
    };

    if is_adjoint {
        lbl.push_str("^\\dagger");
    }

    if !args.is_empty() {
        let joined = args.join(", ");
        lbl = format!("{}({})", lbl, joined);
    }
    lbl
}

fn get_rows_for_operation(
    op: &Operation,
    register_to_row: &RegisterMap,
    is_target: bool,
) -> Vec<usize> {
    let registers = match op {
        Operation::Measurement(m) => {
            if is_target {
                &m.results
            } else {
                &m.qubits
            }
        }
        Operation::Unitary(u) => {
            if is_target {
                &u.targets
            } else {
                &u.controls
            }
        }
        Operation::Ket(k) => {
            if is_target {
                &k.targets
            } else {
                &vec![]
            }
        }
    };

    let mut rows: Vec<usize> = registers
        .iter()
        .filter_map(|reg| register_to_row.get(&(reg.qubit, reg.result)).copied())
        .collect();

    rows.sort();
    rows
}
