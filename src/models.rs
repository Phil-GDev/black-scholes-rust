use statrs::distribution::{ContinuousCDF, Normal};

pub struct BlackScholes {
    pub s: f64,     // Preço do Ativo
    pub k: f64,     // Strike
    pub t: f64,     // Tempo (Anualizado)
    pub r: f64,     // Juros (Contínuos)
    pub sigma: f64, // Volatilidade
}

pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
}

impl BlackScholes {
    pub fn calculate(&self) -> (f64, f64, Greeks) {
        let n = Normal::new(0.0, 1.0).unwrap();
        
        let d1 = ((self.s / self.k).ln() + (self.r + 0.5 * self.sigma.powi(2)) * self.t) 
                 / (self.sigma * self.t.sqrt());
        let d2 = d1 - self.sigma * self.t.sqrt();

        // Preços Teóricos
        let call_price = self.s * n.cdf(d1) - self.k * (-self.r * self.t).exp() * n.cdf(d2);
        let put_price = self.k * (-self.r * self.t).exp() * n.cdf(-d2) - self.s * n.cdf(-d1);

        // Cálculo das Gregas
        let pdf_d1 = (-0.5 * d1.powi(2)).exp() / (2.0 * std::f64::consts::PI).sqrt();
        
        let delta = n.cdf(d1);
        let gamma = pdf_d1 / (self.s * self.sigma * self.t.sqrt());
        let vega = self.s * self.t.sqrt() * pdf_d1 / 100.0;
        let theta = (-(self.s * pdf_d1 * self.sigma) / (2.0 * self.t.sqrt()) 
                    - self.r * self.k * (-self.r * self.t).exp() * n.cdf(d2)) / 252.0;

        (call_price, put_price, Greeks { delta, gamma, theta, vega })
    }
}