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

        let resultControl = MResetZ(control);
        let resultTarget = MResetZ(target);
        return (resultControl, resultTarget);
    }
}