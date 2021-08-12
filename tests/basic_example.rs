
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn got_true() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("true")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(true)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn got_false() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("false")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(false)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn integer_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("1285")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(1285)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn not_true() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("!true")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(false)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn not_false() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("!false")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(true)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }

    #[test]
    fn negative_parses() {
        let int = Interpreter {};

        let ast_node: Vec<AstNode> = parse("-41")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(-41)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), expected_value);
        }
    }
}
