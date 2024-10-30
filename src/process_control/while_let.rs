pub fn disgraceful_destruction() {
  let mut optional = Some(0);

  loop {
    match optional {
      Some(i) => {
        if i > 9 {
          optional = None;
        } else {
          optional = Some(i + 1);
        }
      },
      _ => { break; },
    }
  }
}

pub fn while_let() {
  let mut optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      optional = None;
    } else {
      optional = Some(i + 1);
    }
  }
  
}
