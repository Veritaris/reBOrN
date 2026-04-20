#[macro_export]
macro_rules! ui_select_enum_case {
    ($($ui_var:expr, $mut_var: expr, $enum_case:expr)?) => {
        $(
        $ui_var.selectable_value(
            &mut $mut_var,
            $enum_case,
            $enum_case.to_string(),
        );
        )*
    };
}
pub use ui_select_enum_case;
