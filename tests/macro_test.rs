use feature_gate::feature_gate;

#[feature_gate("test")]
struct Test;

#[test]
fn it_works() {
    let _ = Test {};
}
