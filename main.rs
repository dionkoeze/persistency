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
    for digit in self.digits.iter().rev() {
      println!("{}", digit);
    }
    None
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


fn main() {
  let num = Number::num_from_int(123456);

  num.print();

  println!("2 {}", num.is_div_by_2().unwrap());
  println!("3 {}", num.is_div_by_3().unwrap());
  println!("5 {}", num.is_div_by_5().unwrap());
  println!("7 {}", num.is_div_by_7().unwrap());
}

