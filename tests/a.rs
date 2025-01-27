use publish_test::utils::add;

#[test]
fn feature() {
    let sum = add(1, 1);
    assert_eq!(sum, 2);
}
