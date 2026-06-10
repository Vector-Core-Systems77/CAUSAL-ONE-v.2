use num_complex::Complex64;

type C = Complex64;

pub struct HptOperator {
    pub lambda: f64,
    x0: f64,
    xmax: f64,
    steps: usize,
    pub spectrum_buffer: Vec<f64>,
    pub seed_state: Vec<f64>,
}

impl HptOperator {
    pub fn new() -> Self {
        Self {
            lambda: 0.25,
            x0: 1e-4,
            xmax: 20.0,
            steps: 10000,
            spectrum_buffer: Vec::new(),
            seed_state: vec![0.0; 10],
        }
    }

    fn ode_rhs(&self, x: f64, phi: C, e: f64) -> C {
        C::i() * (e - self.lambda - x.powi(4)) / x * phi
    }

    fn rk4_step(&self, x: f64, phi: C, h: f64, e: f64) -> C {
        let k1 = self.ode_rhs(x, phi, e);
        let k2 = self.ode_rhs(x + h/2.0, phi + h/2.0 * k1, e);
        let k3 = self.ode_rhs(x + h/2.0, phi + h/2.0 * k2, e);
        let k4 = self.ode_rhs(x + h, phi + h * k3, e);
        phi + h/6.0 * (k1 + 2.0*k2 + 2.0*k3 + k4)
    }

    fn solve_for_e(&self, e: f64) -> f64 {
        let h = (self.xmax - self.x0) / self.steps as f64;
        let mut x = self.x0;
        let mut phi = C::new(1.0, 0.0);
        for _ in 0..self.steps {
            phi = self.rk4_step(x, phi, h, e);
            x += h;
        }
        phi.re
    }

    pub fn generate_zero(&mut self, e_guess: f64) -> Option<f64> {
        let mut e_low = e_guess - 5.0;
        let mut e_high = e_guess + 5.0;
        let tol = 1e-8;
        let max_iter = 50;

        let mut f_low = self.solve_for_e(e_low);
        let mut f_high = self.solve_for_e(e_high);

        for _ in 0..10 {
            if f_low * f_high < 0.0 { break; }
            e_low -= 5.0;
            e_high += 5.0;
            f_low = self.solve_for_e(e_low);
            f_high = self.solve_for_e(e_high);
        }

        if f_low * f_high > 0.0 { return None; }

        for _ in 0..max_iter {
            let e_mid = (e_low + e_high) / 2.0;
            let f_mid = self.solve_for_e(e_mid);
            if f_mid.abs() < tol {
                self.spectrum_buffer.push(e_mid);
                self.update_seed();
                return Some(e_mid);
            }
            if f_low * f_mid < 0.0 {
                e_high = e_mid;
                f_high = f_mid;
            } else {
                e_low = e_mid;
                f_low = f_mid;
            }
        }
        None
    }

    fn update_seed(&mut self) {
        let n = self.spectrum_buffer.len().min(10);
        if n == 0 { return; }
        let avg = self.spectrum_buffer[self.spectrum_buffer.len()-n..].iter().sum::<f64>() / n as f64;
        self.seed_state.rotate_left(1);
        self.seed_state[9] = avg;
    }
}

pub fn calculate_causal_effect(_cause: String, _effect: String, lambda: f64) -> f64 {
    let mut hpt = HptOperator::new();
    hpt.lambda = lambda;
    match hpt.generate_zero(lambda) {
        Some(eigenvalue) => eigenvalue,
        None => lambda * 2.5
    }
          }
