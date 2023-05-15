pub trait ScriptUtils {
    fn begin_script() -> Self;
    fn end_script(&mut self) -> Vec<u8>;
    fn load_into_reg(&mut self, target_reg: u8, arg: &dyn Any);
    fn insert_method_args(&mut self, args: &[Box<dyn Any>]);
    fn call_interop(&mut self, method: &str, args: &[Box<dyn Any>]) -> &mut Self;
    fn call_contract(
        &mut self,
        contract_name: &str,
        method: &str,
        args: &[Box<dyn Any>],
    ) -> &mut Self;
}

impl ScriptUtils for ScriptBuilder {
    fn begin_script() -> Self {
        ScriptBuilder::new()
    }

    fn end_script(&mut self) -> Vec<u8> {
        self.emit(Opcode::Ret);
        self.to_script()
    }

    fn load_into_reg(&mut self, target_reg: u8, arg: &dyn Any) {
        // Here you'll need to use `if let` or `match` to check the type of `arg` and call the appropriate `emit_load` function
        // This will be quite verbose as Rust doesn't have a direct equivalent of C#'s `is` keyword
        // For example:
        if let Some(val) = arg.downcast_ref::<String>() {
            self.emit_load(target_reg, val);
        } else if let Some(val) = arg.downcast_ref::<i32>() {
            self.emit_load(target_reg, &BigInt::from(*val));
        } else {
            panic!("Invalid type");
        }
        // And so on for each type...
    }

    fn insert_method_args(&mut self, args: &[Box<dyn Any>]) {
        let temp_reg = 0;
        for arg in args.iter().rev() {
            self.load_into_reg(temp_reg, arg.as_ref());
            self.emit_push(temp_reg);
        }
    }

    fn call_interop(&mut self, method: &str, args: &[Box<dyn Any>]) -> &mut Self {
        self.insert_method_args(args);
        let dest_reg = 0;
        self.emit_load(dest_reg, method);
        self.emit(Opcode::ExtCall, Some(&[dest_reg]));
        self
    }

    fn call_contract(
        &mut self,
        contract_name: &str,
        method: &str,
        args: &[Box<dyn Any>],
    ) -> &mut Self {
        self.insert_method_args(args);
        let temp_reg = 0;
        self.emit_load(temp_reg, method);
        self.emit_push(temp_reg);
        let src_reg = 0;
        let dest_reg = 1;
        self.emit_load(src_reg, contract_name);
        self.emit(Opcode::Ctx, Some(&[src_reg, dest_reg]));
        self.emit(Opcode::Switch, Some(&[dest_reg]));
        self
    }
}
