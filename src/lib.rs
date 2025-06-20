pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_data() -> String {
    "スクレイピング結果".to_string()
}

