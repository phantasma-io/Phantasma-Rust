pub trait GasAllowance {
    fn allow_gas(
        &mut self,
        from: Address,
        to: Address,
        gas_price: BigInt,
        gas_limit: BigInt,
    ) -> &mut Self;
}

impl GasAllowance for ScriptBuilder {
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
}
