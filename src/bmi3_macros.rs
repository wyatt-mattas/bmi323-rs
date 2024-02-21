#[macro_export]
macro_rules! set_bits {
    // Variant without bit shift
    ($reg_data:expr, $idx:expr, $(($mask:expr, $value:expr, $pos:expr)),*) => {{
        set_bits!(@internal $reg_data, $idx, 0, $(($mask, $value, $pos)),*);
    }};
    // Variant with bit shift
    ($reg_data:expr, $idx:expr, $shift:expr, $(($mask:expr, $value:expr, $pos:expr)),*) => {{
        set_bits!(@internal $reg_data, $idx, $shift, $(($mask, $value, $pos)),*);
    }};
    // Internal implementation
    (@internal $reg_data:expr, $idx:expr, $shift:expr, $(($mask:expr, $value:expr, $pos:expr)),*) => {{
        let mut temp: u16 = $reg_data[$idx] as u16;
        $(
            temp = match $pos {
                Some(shift) => (temp & !$mask) | (($value << shift) & $mask),
                None => (temp & !$mask) | ($value & $mask),
            };
        )*
        $reg_data[$idx] = (temp >> $shift) as u8;
    }};
}