fn main() {
    tonic_build::configure()
    .out_dir("src/parallel")
    .compile(&["src/parallel/proto/commnication.proto"], &["src/parallel/"])
    .expect("failed to compile protos");
}