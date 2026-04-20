use crate::signature_parser::parse_jvm_descriptor;

macro_rules! descriptor_transform_tests {
    ($($name:ident: ($descriptor:expr, $signature:expr),)*) => {
        $(
        #[test]
        fn $name() {
            let signature = parse_jvm_descriptor($descriptor).unwrap().to_string();
            assert_eq!(signature, $signature);
        }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    descriptor_transform_tests! {
        simple_test: ("([Ljava/lang/String;[IZ[V)V", "void (java.lang.String[], int[], boolean, void[])"),
    }

    #[test]
    fn test_transform_method_descriptor_into_signature() {
        let descriptor = "([Ljava/lang/String;[IZ[V)V".to_string();
        let signature = &parse_jvm_descriptor(&descriptor).unwrap();
        println!("{}", descriptor);
        println!("{}", &signature);
        assert_eq!(
            "void (java.lang.String[], int[], boolean, void[])",
            signature.to_string()
        )
    }
}
