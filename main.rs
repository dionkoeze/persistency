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

  fn is_zero(&self) -> bool {
    match self.digits.first() {
      Some(0) => true,
      Some(_) => false,
      None => panic!("No digits!")
    }
  }

  fn is_div_by_2(&self) -> bool {
    match self.digits.last() {
      Some(val) => val % 2 == 0,
      None => panic!("No digits!")
    }
  }

  fn is_div_by_3(&self) -> bool {
    if self.digits.len() == 0 {
      panic!("No digits!");
    }

    let mut sum = 0;
    for digit in &self.digits {
      sum += digit;
    }

    sum % 3 == 0
  }

  fn is_div_by_5(&self) -> bool {
    match self.digits.last() {
      Some(0) => true,
      Some(5) => true,
      Some(_) => false,
      None => panic!("No digits!")
    }
  }

  fn is_div_by_7(&self) -> bool {
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

    check % 7 == 0
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
    let mut pf = PrimeFactorization::new();

    find_factorization(num, &mut pf);

    pf
  }

  fn add(&mut self, prime: i16) {
    self.primes.push(prime);
  }

  fn print(&self) {
    for prime in &self.primes {
      println!("{}", prime);
    }
  }
}


fn find_factorization(num: Number, mut pf: &mut PrimeFactorization) {
  if num.is_div_by_2() {
    pf.add(2);
    find_factorization(num.div_by_2(), &mut pf);
  } else if num.is_div_by_3() {
    pf.add(3);
    find_factorization(num.div_by_3(), &mut pf);
  } else if num.is_div_by_5() {
    pf.add(5);
    find_factorization(num.div_by_5(), &mut pf);
  } else if num.is_div_by_7() {
    pf.add(7);
    find_factorization(num.div_by_7(), &mut pf);
  }
}


fn main() {
  let num = Number::num_from_int(338688);

  num.print();

  println!("2 {}", num.is_div_by_2());
  println!("3 {}", num.is_div_by_3());
  println!("5 {}", num.is_div_by_5());
  println!("7 {}", num.is_div_by_7());

  num.div_by_2().print();
  num.div_by_3().print();
  num.div_by_5().print();
  num.div_by_7().print();

  let pf = PrimeFactorization::from_num(num);
  pf.print();
}

