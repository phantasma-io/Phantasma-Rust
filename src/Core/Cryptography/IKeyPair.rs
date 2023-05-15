pub trait IKeyPair {
    fn private_key(&self) -> Vec<u8>;
    fn public_key(&self) -> Vec<u8>;

    // This is a method that takes a closure as an argument, similar to the Func<> delegate in C#
    /*fn sign<F>(&self, msg: Vec<u8>, custom_sign_function: Option<F>) -> Signature
    where
        F: Fn(Vec<u8>, Vec<u8>, Vec<u8>) -> Vec<u8>;*/
}
