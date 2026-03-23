pub mod Object;
pub mod Scanner;
pub mod Token;
pub mod TokenType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sample_source = vec!["+"];
        let scanner = Scanner::LoxScanner{
            source: &sample_source, 
            Tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scanToken();
        assert_eq!(scanner.Tokens.pop(), TokenType::PLUS); 
    }
}
