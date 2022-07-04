#[derive(Debug)]
pub struct Flags {
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub b: bool,
    pub v: bool,
    pub n: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }

    pub fn trigger(aflags: Vec<char>) -> Flags {
        let mut af = Flags::new();
        for flag in aflags {
            match flag {
                'C' => af.c = true,
                'Z' => af.z = true,
                'I' => af.i = true,
                'D' => af.d = true,
                'B' => af.b = true,
                'V' => af.v = true,
                'N' => af.n = true,
                _ => (),
            }
        }
        af
    }

    pub fn trigger_all() -> Flags {
        Flags {
            c: true,
            z: true,
            i: true,
            d: true,
            b: true,
            v: true,
            n: true,
        }
    }

    pub fn trig_c_if(&mut self, condition: bool) {
        if condition {
            self.c = true;
        } else {
            self.c = false;
        }
    }

    pub fn trig_z_if(&mut self, condition: bool) {
        if condition {
            self.z = true;
        } else {
            self.z = false;
        }
    }

    pub fn trig_i_if(&mut self, condition: bool) {
        if condition {
            self.i = true;
        } else {
            self.i = false;
        }
    }

    pub fn trig_d_if(&mut self, condition: bool) {
        if condition {
            self.d = true;
        } else {
            self.d = false;
        }
    }

    pub fn trig_b_if(&mut self, condition: bool) {
        if condition {
            self.b = true;
        } else {
            self.b = false;
        }
    }

    pub fn trig_v_if(&mut self, condition: bool) {
        if condition {
            self.v = true;
        } else {
            self.v = false;
        }
    }

    pub fn trig_n_if(&mut self, condition: bool) {
        if condition {
            self.n = true;
        } else {
            self.n = false;
        }
    }
}
