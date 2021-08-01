use dlhn_bench::Test;
use iai::main;

fn serialize_json() {
    let buf = Vec::new();
    serde_json::to_writer(buf, &Test::default()).unwrap();
}

fn deserialize_json() {
    let buf = [
        123, 34, 97, 34, 58, 116, 114, 117, 101, 44, 34, 98, 34, 58, 50, 53, 53, 44, 34, 99, 34,
        58, 54, 53, 53, 51, 53, 44, 34, 100, 34, 58, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 44,
        34, 101, 34, 58, 49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54,
        49, 53, 44, 34, 102, 34, 58, 45, 49, 50, 56, 44, 34, 103, 34, 58, 45, 51, 50, 55, 54, 56,
        44, 34, 104, 34, 58, 45, 50, 49, 52, 55, 52, 56, 51, 54, 52, 56, 44, 34, 105, 34, 58, 45,
        57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56, 44, 34, 106,
        34, 58, 51, 46, 52, 48, 50, 56, 50, 51, 53, 101, 51, 56, 44, 34, 107, 34, 58, 49, 46, 55,
        57, 55, 54, 57, 51, 49, 51, 52, 56, 54, 50, 51, 49, 53, 55, 101, 51, 48, 56, 44, 34, 108,
        34, 58, 34, 116, 101, 115, 116, 34, 44, 34, 109, 34, 58, 91, 116, 114, 117, 101, 44, 102,
        97, 108, 115, 101, 44, 116, 114, 117, 101, 44, 102, 97, 108, 115, 101, 93, 44, 34, 110, 34,
        58, 123, 34, 97, 34, 58, 116, 114, 117, 101, 44, 34, 98, 34, 58, 102, 97, 108, 115, 101,
        44, 34, 99, 34, 58, 116, 114, 117, 101, 44, 34, 100, 34, 58, 102, 97, 108, 115, 101, 125,
        125,
    ];
    let _: Test = serde_json::from_reader(buf.as_ref()).unwrap();
}

main!(serialize_json, deserialize_json,);
