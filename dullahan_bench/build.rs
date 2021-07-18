fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(&["."]);
    config.compile_protos(&["src/proto_test.proto"], &["src/"]).unwrap();
}
