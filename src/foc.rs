fn clarke_transform(current_a: f32, current_b: f32) -> (f32, f32) {
    let current_alpha: f32 = current_a;
    let current_beta: f32 = (current_a + 2*current_b) / (3.0).sqrt();
    (current_alpha, current_beta)
}

fn park_transform(current_alpha: f32, current_beta: f32, angle: f32) -> (f32, f32) {
    let current_d: f32 = current_alpha * angle.cos() + current_beta * angle.sin();
    let current_q: f32 = current_beta * angle.cos() - current_alpha * angle.sin();
    (current_d, current_q)
}

fn inv_park_transform(current_d: f32, current_q: f32, angle: f32) {
    let current_alpha: f32 = current_d * angle.cos() - current_q * angle.sin();
    let current_beta: f32 = current_q * angle.cos() + current_d * angle.sin();
    (current_alpha, current_beta)
}

fn svpwm_gen(current_alpha: f32, current_beta: f32, max_pwm: u32) -> (u32, u32, u32) {
    //TODO Scale everything by max PWM?
    let pwm_a: u32 = current_alpha;
    let pwm_b: u32 = (-current_alpha + ((3.0).sqrt() * current_beta)) / 2.0 as u32;
    let pwm_c: u32 = (-current_alpha - ((3.0).sqrt() * current_beta)) / 2.0 as u32;
    (pwm_a, pwm_b, pwm_c)
}
