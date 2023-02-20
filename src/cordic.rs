use stm32ral::{cordic, read_reg, write_reg, modify_reg};

fn float_to_q31(input: f32) -> i32 {
    let out = input * 2147483648.0;
    let out_round;
    if out > 0.0 {
        out_round = out + 0.5;
    } else {
        out_round = out - 0.5;
    }
    out_round as i32
}

fn q31_to_float(input: i32) -> f32 {
    let out = input as f32 / 2147483648.0;
    out
}

pub struct Cordic {
    cordic: cordic::Instance,
}

impl Cordic {
    pub fn new(cordic: cordic::Instance) -> Self {
        Self { cordic }
    }

    pub fn init(&self) {
        // Init sin function
        write_reg!(cordic, self.cordic, CSR, NARGS: Num2, NRES: Num2,
                                            PRECISION: 0b0101, FUNC: Sine);

        // Dummy calculation
        write_reg!(cordic, self.cordic, WDATA, 0x80000000);
        write_reg!(cordic, self.cordic, WDATA, 0x7FFFFFFF);

        let _dummy = read_reg!(cordic, self.cordic, RDATA);
        let _dummy = read_reg!(cordic, self.cordic, RDATA);
        modify_reg!(cordic, self.cordic, CSR, NARGS: Num1);
    }

    pub fn calc_sin_cos(&self, theta: f32) -> (f32, f32) {
        let fxd_input = float_to_q31(theta);
        write_reg!(cordic, self.cordic, WDATA, fxd_input as u32);
        let fxd_sin = read_reg!(cordic, self.cordic, RDATA);
        let fxd_cos = read_reg!(cordic, self.cordic, RDATA);
        (q31_to_float(fxd_sin as i32), q31_to_float(fxd_cos as i32))
    }

    pub fn calc_sin_cos_deferred(&self, theta: f32) {
        let fxd_input = float_to_q31(theta);
        write_reg!(cordic, self.cordic, WDATA, fxd_input as u32);
    }

    pub fn get_result(&self) -> (f32, f32) {
        let fxd_sin = read_reg!(cordic, self.cordic, RDATA);
        let fxd_cos = read_reg!(cordic, self.cordic, RDATA);
        (q31_to_float(fxd_sin as i32), q31_to_float(fxd_cos as i32))
    }
}
