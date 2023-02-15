fn main() {
    uniffi::generate_scaffolding("./src/wallet_ops.udl").unwrap();
}
