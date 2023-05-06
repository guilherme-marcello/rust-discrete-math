struct ModInteger {
    value:  i32,    // value of the modular integer
    n:      i32     // modulus for arithmetic operations
}

impl ModInteger {
    pub fn new(value: i32, modulus: i32) -> Self {
        assert!(modulus > 0, "Modulus must be positive");
        ModInteger {
            value: value % modulus,
            n: modulus,
        }
    }

    fn _add(&self, other: &ModInteger, signal: i32) -> ModInteger {
        assert!(self.n == other.n, "Both modulus must be equal");
        ModInteger::new(self.value + signal*other.value, self.n)
    }

    pub fn add(&self, other: &ModInteger) -> ModInteger {
        self._add(other, 1)
    }

    pub fn sub(&self, other: &ModInteger) -> ModInteger {
        self._add(other, -1)
    }

    pub fn mul(&self, value: i32) -> ModInteger {
        ModInteger::new(self.value * value, self.n)
    }

    pub fn pow(&self, power: u32) -> ModInteger {
        ModInteger::new(self.value.pow(power), self.n)
    }

    pub fn to_string(&self) -> String {
        format!("{} mod {}", &self.value, &self.n)
    }
}

fn main() {
    let a = ModInteger::new(1, 3);
    let b = ModInteger::new(11, 3);
    println!("a = {}", a.to_string());
    println!("b = {}", b.to_string());
    println!("a + b = {}", a.add(&b).to_string());
    println!("b - a = {}", b.sub(&a).to_string());
    println!("a*6 = {}", a.mul(6).to_string());
    println!("b**2 = {}", b.pow(2).to_string());
}