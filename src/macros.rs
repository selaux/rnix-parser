#[macro_export]
macro_rules! nix_inner {
    ({
        $($ident:ident = ($($val:tt)*);)*
    }) => {{
        AST::Set(vec![
            $((String::from(stringify!($ident)), nix_inner!($($val)*))),*
        ])
    }};
    (let {
        $($ident:ident = ($($val:tt)*);)*
    } in $($remaining:tt)*) => {{
        AST::LetIn(
            vec![$((String::from(stringify!($ident)), nix_inner!($($val)*))),*],
            Box::new(nix_inner!($($remaining)*))
        )
    }};
    (with ($($namespace:tt)*); $($remaining:tt)*) => {{
        AST::With(Box::new((
            nix_inner!($($namespace)*),
            nix_inner!($($remaining)*)
        )))
    }};
    (import ($($path:tt)*)) => {{
        AST::Import(Box::new(nix_inner!($($path)*)))
    }};
    (($($val1:tt)*) + ($($val2:tt)*)) => {{
        AST::Add(Box::new((nix_inner!($($val1)*), nix_inner!($($val2)*))))
    }};
    (($($val1:tt)*) - ($($val2:tt)*)) => {{
        AST::Sub(Box::new((nix_inner!($($val1)*), nix_inner!($($val2)*))))
    }};
    (($($val1:tt)*) * ($($val2:tt)*)) => {{
        AST::Mul(Box::new((nix_inner!($($val1)*), nix_inner!($($val2)*))))
    }};
    (($($val1:tt)*) / ($($val2:tt)*)) => {{
        AST::Div(Box::new((nix_inner!($($val1)*), nix_inner!($($val2)*))))
    }};
    ($val:ident) => {{
        AST::Var(String::from(stringify!($val)))
    }};
    (./$val:expr) => {{
        AST::Value(Value::Path(Anchor::Relative, String::from($val)))
    }};
    ($val:expr) => {{
        AST::Value(Value::from($val))
    }};
}
#[macro_export]
macro_rules! nix {
    ($($tokens:tt)*) => {{
        #[allow(unused_imports)]
        use crate::{
            parser::AST,
            value::{Anchor, Value}
        };
        nix_inner!($($tokens)*)
    }}
}

#[cfg(test)]
#[test]
fn test_macro() {
    use crate::parser::AST;
    assert_eq!(
        nix!({
            string = ("Hello World");
            number = ((3) * ((4) + (2)));
        }),
        AST::Set(vec![
            ("string".into(), AST::Value("Hello World".into())),
            ("number".into(), AST::Mul(Box::new((
                AST::Value(3.into()),
                AST::Add(Box::new((
                    AST::Value(4.into()),
                    AST::Value(2.into()),
                ))),
            )))),
        ])
    );
}