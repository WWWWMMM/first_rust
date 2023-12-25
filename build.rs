fn main() {
    tonic_build::compile_protos("src/parallel/service.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}