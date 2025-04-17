use std::fmt;
use crate::qsim_core::gate::{QuantumGate, GateOp};
use crate::qsim_core::qubit::Qubit;

/// Represents a quantum circuit that can be built and executed
/// 
/// A quantum circuit is a sequence of quantum gates applied to one or more qubits.
/// This implementation allows for building circuits incrementally and executing them
/// on a set of qubits.
pub struct QuantumCircuit {
    /// The number of qubits in the circuit
    num_qubits: usize,
    /// The sequence of gate operations to apply
    operations: Vec<GateOp>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with the specified number of qubits
    /// 
    /// # Arguments
    /// * `num_qubits` - The number of qubits in the circuit
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);  // Create a 1-qubit circuit
    /// ```
    pub fn new(num_qubits: usize) -> Self {
        QuantumCircuit {
            num_qubits,
            operations: Vec::new(),
        }
    }

    /// Applies a Hadamard gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.h(0);  // Apply Hadamard gate to the first qubit
    /// ```
    pub fn h(&mut self, target: usize) {
        self.add_gate(QuantumGate::H, target);
    }

    /// Applies a Pauli-X gate to the specified qubit
    pub fn x(&mut self, target: usize) {
        self.add_gate(QuantumGate::X, target);
    }

    /// Applies a Pauli-Y gate to the specified qubit
    pub fn y(&mut self, target: usize) {
        self.add_gate(QuantumGate::Y, target);
    }

    /// Applies a Pauli-Z gate to the specified qubit
    pub fn z(&mut self, target: usize) {
        self.add_gate(QuantumGate::Z, target);
    }

    /// Applies an S gate to the specified qubit
    pub fn s(&mut self, target: usize) {
        self.add_gate(QuantumGate::S, target);
    }

    /// Applies a T gate to the specified qubit
    pub fn t(&mut self, target: usize) {
        self.add_gate(QuantumGate::T, target);
    }

    /// Applies a CNOT gate with the specified control and target qubits
    /// 
    /// # Arguments
    /// * `control` - The index of the control qubit
    /// * `target` - The index of the target qubit
    /// 
    /// # Panics
    /// Panics if either qubit index is out of bounds
    pub fn cnot(&mut self, control: usize, target: usize) {
        if control >= self.num_qubits || target >= self.num_qubits {
            panic!("Qubit index out of bounds for circuit with {} qubits", self.num_qubits);
        }
        let step = self.operations.iter()
            .filter(|op| op.target == target)
            .map(|op| op.step)
            .max()
            .map(|s| s + 1)
            .unwrap_or(0);
        self.operations.push(GateOp::controlled(QuantumGate::CNOT, control, target, step));
    }

    /// Adds a gate operation to the circuit
    /// 
    /// # Arguments
    /// * `gate` - The quantum gate to apply
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Panics
    /// Panics if the target qubit index is out of bounds
    fn add_gate(&mut self, gate: QuantumGate, target: usize) {
        if target >= self.num_qubits {
            panic!("Qubit index {} is out of bounds for circuit with {} qubits", 
                   target, self.num_qubits);
        }
        let step = self.operations.iter()
            .filter(|op| op.target == target)
            .map(|op| op.step)
            .max()
            .map(|s| s + 1)
            .unwrap_or(0);
        self.operations.push(GateOp::new(gate, target, step));
    }

    /// Executes the circuit on a set of qubits
    /// 
    /// # Arguments
    /// * `qubits` - A slice of qubits to apply the circuit to
    /// 
    /// # Panics
    /// Panics if the number of qubits doesn't match the circuit size
    /// 
    /// # Examples
    /// ```
    /// use intrico::{QuantumCircuit, Qubit};
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.h(0);
    /// 
    /// let mut qubits = vec![Qubit::zero()];
    /// qc.execute(&mut qubits);
    /// ```
    pub fn execute(&self, qubits: &mut [Qubit]) {
        if qubits.len() != self.num_qubits {
            panic!("Number of qubits ({}) doesn't match circuit size ({})", 
                   qubits.len(), self.num_qubits);
        }

        for op in &self.operations {
            qubits[op.target].apply(op.gate);
        }
    }

    /// Returns the number of qubits in the circuit
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of operations in the circuit
    pub fn num_operations(&self) -> usize {
        self.operations.len()
    }

    /// Displays the quantum circuit in ASCII format to stdout
    pub fn display(&self) {
        let height = 2 * self.num_qubits - 1;
        let mut lines = vec![String::new(); height];

        
        

        // Print the circuit
        for line in lines {
            println!("{}", line);
        }
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quantum Circuit ({} qubits, {} operations):", 
                 self.num_qubits, self.num_operations())?;
        for (i, op) in self.operations.iter().enumerate() {
            writeln!(f, "  {}. {} on qubit {} (Step: {})", i + 1, op.gate, op.target, op.step)?;
        }
        Ok(())
    }
}

impl fmt::Debug for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QuantumCircuit {{ num_qubits: {}, operations: {:?} }}", 
               self.num_qubits, self.operations)
    }
} 