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

#[macro_export]
macro_rules! from_reg_data {
    ($name:ident, $x_mask:expr, $y_mask:expr, $y_pos:expr, $z_mask:expr, $z_pos:expr) => {
        fn $name(reg_data: &[u16]) -> Self {
            Self {
                x: reg_data[0] as i16,
                y: reg_data[1] as i16,
                z: reg_data[2] as i16,
                sens_time: reg_data[3] as u32 | ((reg_data[4] as u32) << 16),
                sat_x: reg_data[5] as u8 & $x_mask as u8,
                sat_y: (reg_data[5] as u8 & $y_mask as u8) >> $y_pos,
                sat_z: (reg_data[5] as u8 & $z_mask as u8) >> $z_pos,
            }
        }
    };
}

// #[macro_export]
// macro_rules! check_boundary_val {
//     ($self:expr, $val:expr, $min:expr, $max:expr, $min_flag:expr, $max_flag:expr) => {{
//         |val: Option<&mut u8>, min: u8, max: u8| -> Bmi3Result<()> {
//             if let Some(v) = val {
//                 if *v < min {
//                     *v = min;
//                     $self.info |= $min_flag; // Use passed flag for min value
//                 }

//                 if *v > max {
//                     *v = max;
//                     $self.info |= $max_flag; // Use passed flag for max value
//                 }

//                 Ok(())
//             } else {
//                 Err(Bmi3Error::NullPtr)
//             }
//         }($val, $min, $max)
//     }};
// }
