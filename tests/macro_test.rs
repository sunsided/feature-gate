use feature_gate::feature_gate;

// #[feature_gate(any(feature = "test", not(feature = "test")))]
// #[feature_gate(feature = "test")]
#[feature_gate("test")]
struct Test;

#[test]
fn it_works() {
    let _ = Test {};
}
