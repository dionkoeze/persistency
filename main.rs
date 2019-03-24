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
    // 1 3 2 6 4 5
    unimplemented!()
  }

  fn div_by_2(&self) -> Number {
    unimplemented!()
  }

  fn div_by_3(&self) -> Number {
    unimplemented!()
  }

  fn div_by_5(&self) -> Number {
    unimplemented!()
  }

  fn div_by_7(&self) -> Number {
    unimplemented!()
  }

  fn print(&self) {
    println!("Dit is een number");
    for digit in &self.digits {
      println!("{}", digit);
    }
  }
}


fn int_to_vec(num: i64, vec: &mut Vec<i16>) {
  println!("----------");
  println!("{}", num);
  if num != 0 {
    int_to_vec(num/10, vec);
    vec.push((num % 10) as i16);
  } 
}


fn main() {
  println!("hoi");

  let num = Number::num_from_int(0);

  num.print();
}

