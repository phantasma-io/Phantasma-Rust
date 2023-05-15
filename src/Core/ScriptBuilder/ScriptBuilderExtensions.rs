pub trait ScriptBuilderExtensions {
    fn allow_gas(
        &mut self,
        from: Address,
        to: Address,
        gas_price: BigInt,
        gas_limit: BigInt,
    ) -> &mut Self;

    fn spend_gas(&mut self, from: Address) -> &mut Self;

    fn transfer_tokens(
        &mut self,
        token: String,
        from: Address,
        to: Address,
        amount: BigInt,
    ) -> &mut Self;

    fn call_contract(&mut self, contract: NativeContractKind, method: &str) -> &mut Self;
}

impl ScriptBuilderExtensions for ScriptBuilder {
    fn allow_gas(
        &mut self,
        from: Address,
        to: Address,
        gas_price: BigInt,
        gas_limit: BigInt,
    ) -> &mut Self {
        self.call_contract(
            NativeContractKind::Gas,
            "AllowGas",
            from,
            to,
            gas_price,
            gas_limit,
        )
    }

    fn spend_gas(&mut self, from: Address) -> &mut Self {
        self.call_contract(NativeContractKind::Gas, "SpendGas", from)
    }

    fn transfer_tokens(&mut self, from: Address) -> &mut Self {
        self.call_interop(NativeContractKind::Gas, "SpendGas", from)
    }
}
