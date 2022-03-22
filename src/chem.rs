pub struct Param {
    pub val: f64,  // Value; starts with 0.0
    pub var: f64,  // Variation; starts with: 0.0
}

pub struct Radical {
    pub lwa: Param,  // Line width A
    // pub lwb: Param,
    // pub lwc: Param,
    pub lrtz: Param,  // Lorentzian linewidth parameter (%)
    pub amount: Param,  // Relative amount
    pub dh1: Param,
    // pub nucs: Vec<Nucleus>,
}
