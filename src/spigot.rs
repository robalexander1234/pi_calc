pub struct Spigot {
    pub result: Vec<i32>,
    pub digits: Vec<i32>,
    nn: usize,
    len: usize,
}

impl Spigot {
    //------------------------------------------------------------
    // Constructor
    //------------------------------------------------------------
    pub fn new(size: usize) -> Self {
        let nn: usize = size;
        let len: usize = (10 * nn) / 3 + 1;
        let digits: Vec<i32> = vec![2; len];
        Spigot {
            result: Vec::with_capacity(nn),
            digits,
            nn,
            len,
        }
    }
    //------------------------------------------------------------
    // find_digits
    //------------------------------------------------------------
    pub fn find_digits(&mut self) {
        let mut carry: i32;
        let mut first = true;
        let mut nines: i32 = 0;
        let mut predigit: i32 = 0;

        for _jj in 0..self.nn + 1 {
            carry = 0;

            // Sweep right to left
            let mut ii = self.len - 1;
            while ii > 0 {
                let den = 2 * ii + 1;
                let temp = self.digits[ii] * 10 + carry;
                self.digits[ii] = temp % den as i32;
                carry = (temp / den as i32) * ii as i32;
                ii -= 1;
            }

            self.digits[0] = self.digits[0] * 10 + carry;
            let digit = self.digits[0] / 10;
            self.digits[0] = self.digits[0] % 10;

            if digit < 9 {
                if !first {
                    self.result.push(predigit);
                    for _k in 0..nines {
                        self.result.push(9);
                    }
                }
                first = false;
                predigit = digit;
                nines = 0;
            } else if digit == 9 {
                nines += 1;
            } else {
                // digit == 10, carry ripple
                self.result.push(predigit + 1);
                for _k in 0..nines {
                    self.result.push(0);
                }
                predigit = 0;
                nines = 0;
            }
        }

        // Flush last predigit
        self.result.push(predigit);
    }

    //------------------------------------------------------------
    // histogram
    //------------------------------------------------------------
    pub fn histogram(&self) {
        let mut histogram = [0i32; 10];
        for ii in 0..self.result.len() {
            let dig: usize = self.result[ii] as usize;
            histogram[dig] += 1;
        }
        for i in 0..10 {
            println!(
                "  {}: {} ({:.2}%)",
                i,
                histogram[i],
                100.0 * histogram[i] as f64 / self.result.len() as f64
            );
        }
    }
}
