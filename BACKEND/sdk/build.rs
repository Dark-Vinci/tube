
fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./src/generated_proto_rs")
        .compile(
            &[
                "./src/proto/auth/auth.proto",
            ],
            &["./src/proto"]
        )
        .unwrap();

    //add more for other services
}