pub struct Bmi3MapInt {
    /*  Map interrupt output to either INT1 or INT2 or IBI
     *  Value   Name        Description
     *   00   DISABLED   Interrupt disabled
     *   01   MAP_INT1     Mapped to INT1
     *   10   MAP_INT2     Mapped to INT2
     *   11   MAP_IBI     Mapped to I3C IBI
     */
    /// Maps no-motion output to either INT1 or INT2 or IBI
    pub no_motion_out: u8,

    /// Maps any-motion output to either INT1 or INT2 or IBI
    pub any_motion_out: u8,

    /// Maps flat output to either INT1 or INT2 or IBI
    pub flat_out: u8,

    /// Maps orientation output to either INT1 or INT2 or IBI
    pub orientation_out: u8,

    /// Maps step detector output to either INT1 or INT2 or IBI
    pub step_detector_out: u8,

    /// Maps step counter output to either INT1 or INT2 or IBI
    pub step_counter_out: u8,

    /// Maps significant motion output to either INT1 or INT2 or IBI
    pub sig_motion_out: u8,

    /// Maps tilt output to either INT1 or INT2 or IBI
    pub tilt_out: u8,

    /// Maps tap output to either INT1 or INT2 or IBI
    pub tap_out: u8,

    /// Maps i3c output to either INT1 or INT2 or IBI  
    pub i3c_out: u8,

    /// Maps feature engine's error or status change to either INT1 or INT2 or IBI
    pub err_status: u8,

    /// Maps temperature data ready interrupt to either INT1 or INT2 or IBI
    pub temp_drdy_int: u8,

    /// Maps gyro data ready interrupt to either INT1 or INT2 or IBI
    pub gyr_drdy_int: u8,

    /// Maps accel data ready interrupt to either INT1 or INT2 or IBI
    pub acc_drdy_int: u8,

    /// Maps FIFO watermark interrupt to either INT1 or INT2 or IBI
    pub fifo_watermark_int: u8,

    /// Maps FIFO full interrupt to either INT1 or INT2 or IBI
    pub fifo_full_int: u8,
}

impl Bmi3MapInt {
    pub fn default() -> Self {
        Bmi3MapInt {
            no_motion_out: 0,
            any_motion_out: 0,
            flat_out: 0,
            orientation_out: 0,
            step_detector_out: 0,
            step_counter_out: 0,
            sig_motion_out: 0,
            tilt_out: 0,
            tap_out: 0,
            i3c_out: 0,
            err_status: 0,
            temp_drdy_int: 0,
            gyr_drdy_int: 0,
            acc_drdy_int: 0,
            fifo_watermark_int: 0,
            fifo_full_int: 0,
        }
    }
}