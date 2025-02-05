use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_example() {
        assert_eq!(1 + 1, 2);
    }
}
