#[derive(Debug)]
pub struct Flags {
    c: bool,
    z: bool,
    i: bool,
    d: bool,
    b: bool,
    v: bool,
    n: bool,
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
}
