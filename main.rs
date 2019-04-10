struct Number {
    digits: Vec<i16>
}


impl Number {
  fn new() -> Self {
    Self {
        digits: Vec::new()
    }
  }

  fn num_from_int(num: i64) -> Self {
    let mut digits = Vec::new();

    if num == 0 {
      digits.push(0)
    } else {
      int_to_vec(num, &mut digits);
    }

    Self {
      digits: digits
    }
  }

  fn is_zero(&self) -> Option<bool> {
    if let Some(0) = self.digits.first() {
      if self.digits.len() == 1 {
        Some(true)
      } else {
        Some(false)
      }
    } else {
      None
    }
  }

  fn is_div_by_2(&self) -> Option<bool> {
    match self.digits.last() {
      Some(val) => Some(val % 2 == 0),
      None => None
    }
  }

  fn is_div_by_3(&self) -> Option<bool> {
    let mut sum = 0;
    for digit in &self.digits {
      sum += digit;
    }

    Some(sum % 3 == 0)
  }

  fn is_div_by_5(&self) -> Option<bool> {
    match self.digits.last() {
      Some(0) => Some(true),
      Some(5) => Some(true),
      Some(_) => Some(false),
      None => None
    }
  }

  fn is_div_by_7(&self) -> Option<bool> {
    let mut iter = self.digits.iter().rev();
    let mut check: i16 = 0;
    let mut flip: i16 = 1;

    loop {
      let mut trip: i16 = 0;

      if let Some(i) = iter.next() {
        trip += i;
      } else {
        break;
      }

      if let Some(i) = iter.next() {
        trip += i * 10;
      }

      if let Some(i) = iter.next() {
        trip += i * 100;
      }

      check += flip * trip;
      flip *= -1;
    }

    Some(check % 7 == 0)
  }

  fn div_by(&self, n: i16) -> Self {
    let mut result = Number::new();
    let mut carry: i16 = 0;

    for digit in &self.digits {
      let new_digit = (digit + carry) / n;

      if new_digit != 0 {
        result.digits.push(new_digit);
      }

      carry += digit - new_digit * n;
      carry *= 10;
    }

    if result.digits.len() == 0 {
      result.digits.push(0);
    }

    result
  }

  fn div_by_2(&self) -> Self {
    self.div_by(2)
  }

  fn div_by_3(&self) -> Self {
    self.div_by(3)
  }

  fn div_by_5(&self) -> Self {
    self.div_by(5)
  }

  fn div_by_7(&self) -> Self {
    self.div_by(7)
  }

  fn print(&self) {
    let mut repr: String = "".to_string();

    for digit in self.digits.iter() {
      repr += &digit.to_string();
    }

    println!("{}", repr);
  }
}


fn int_to_vec(num: i64, vec: &mut Vec<i16>) {
  // println!("----------");
  // println!("{}", num);
  if num != 0 {
    int_to_vec(num/10, vec);
    vec.push((num % 10) as i16);
  } 
}


struct PrimeFactorization {
  primes: Vec<i16>
}


impl PrimeFactorization {
  fn new() -> Self {
    PrimeFactorization {
      primes: Vec::new()
    }
  }

  fn from_num(num: Number) -> Self {
    let pf = PrimeFactorization::new();

    find_factorization(num, pf);

    pf
  }

  fn add(&mut self, prime: i16) {
    primes.push(prime);
  }
}


fn find_factorization(num: Number, pf: &mut PrimeFactorization) {
  if let Some(b) = num.is_div_by_2() {
    pf.add(2);
    find_factorization(num.div_by_2(), pf);
  } else if num.is_div_by_3() {
    pf.add(3);
  } else if num.is_div_by_5() {
    pf.add(5);
  } else if num.is_div_by_7() {
    pf.add(7);
  }
}


fn main() {
  let num = Number::num_from_int(6976984);

  num.print();

  println!("2 {}", num.is_div_by_2().unwrap());
  println!("3 {}", num.is_div_by_3().unwrap());
  println!("5 {}", num.is_div_by_5().unwrap());
  println!("7 {}", num.is_div_by_7().unwrap());

  num.div_by_2().print();
  num.div_by_3().print();
  num.div_by_5().print();
  num.div_by_7().print();

  
}

