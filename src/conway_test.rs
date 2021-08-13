#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn got_true() {
        let mut int = Interpreter {
            env: Environment::new(),
        };

        let ast_node: Vec<AstNode> = parser("true")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(true)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn got_false() {
        let mut int = Interpreter {
            env: Environment::new(),
        };

        let ast_node: Vec<AstNode> = parser("false")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(false)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn integer_parses() {
        let mut int = Interpreter {
            env: Environment::new(),
        };
        let ast_node: Vec<AstNode> = parser("1285")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(1285)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn not_true() {

        let mut int = Interpreter { env: Environment::new() };
        let ast_node: Vec<AstNode> = parser("!true")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(false)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn not_false() {
        let mut int = Interpreter { env: Environment::new() };

        let ast_node: Vec<AstNode> = parser("!false")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Boolean(true)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn negative_parses() {
        let mut int = Interpreter { env: Environment::new() };
        let ast_node: Vec<AstNode> = parser("-41")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(-41)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn variable_is_nil() {
        let mut int = Interpreter { env: Environment::new() };
        let ast_node: Vec<AstNode> = parser("var a")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Nil];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn variable_is_reassigned() {
        let mut int = Interpreter { env: Environment::new() };
        let ast_node: Vec<AstNode> = parser("let a = 6\n 
            a = 4
            \n
            a")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected_nodes = vec![Value::Int(6), Value::Int(4), Value::Int(4)];
        for (node, expected_value) in ast_node.into_iter().zip(expected_nodes) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }

    #[test]
    fn variable_is_assigned() {
        let mut int = Interpreter { env: Environment::new() };
        let ast_node: Vec<AstNode> = parser("let a = 10")
            .map_err(|e| format!("An error has ocurred {}", e))
            .unwrap();

        let expected = vec![Value::Int(10)];
        for (node, expected_value) in ast_node.into_iter().zip(expected) {
            assert_eq!(int.eval(&node), Ok(expected_value));
        }
    }
}
