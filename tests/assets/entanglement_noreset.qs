namespace Demos {

    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Measurement;
    open Microsoft.Quantum.Diagnostics;

    @EntryPoint()
    operation Run() : (Result, Result) {
        use (control, target) = (Qubit(), Qubit());

        H(control);
        CNOT(control, target);
        
        DumpMachine();

        let resultControl = M(control);
        let resultTarget = M(target);
        return (resultControl, resultTarget);
    }
}