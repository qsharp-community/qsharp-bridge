namespace MyQuantumApp {
    
    @EntryPoint()
    operation Main() : Bool {
        use q = Qubit();
        let m = M(q);
        IsLossResult(m)
    }
}