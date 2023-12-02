use base64;

pub fn encode_to_base64(data: &str) -> String{
    base64::encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_to_base64() {
        let input = "ALBNM, PROD001, 12, 2023-01-01";
        let expected_output = "QUxCTk0sIFBST0QwMDEsIDEyLCAyMDIzLTAxLTAx";
        let result = encode_to_base64(input);
        assert_eq!(result, expected_output);
    }
}
