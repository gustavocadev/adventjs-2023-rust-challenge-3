fn find_naughty_step(original: String, modified: String) -> String {
  let mut modified_step = String::new();
  let modified_characters: Vec<char> = modified.chars().collect();

  let len_modified = modified.clone().chars().collect::<Vec<char>>();
  let len_original = original.clone().chars().collect::<Vec<char>>();

  // find hidden steps
  if len_original.len() < len_modified.len() {
    let mut deleted_chars = 0;
    for (idx, original_char) in original.chars().enumerate() {
      for modified_char in modified_characters[idx + deleted_chars..].iter() {
        if original_char != *modified_char {
          modified_step.push(*modified_char);
          deleted_chars += 1;
        } else {
          break;
        }
      }
    }

    println!("{:?}", modified_characters);

    // find the rest
    let modified_characters: Vec<char> = modified_characters
      .iter()
      .filter(|&&modified_char| !modified_step.contains(modified_char))
      .cloned()
      .collect();

    for rest in modified_characters[original.len()..].iter() {
      modified_step.push(*rest);
    }

    return modified_step;
  }

  // Doing the inverse
  let modified_characters: Vec<char> = original.chars().collect();
  let original: Vec<char> = modified.chars().collect();

  let mut deleted_chars = 0;
  for (idx, original_char) in original.iter().enumerate() {
    for modified_char in modified_characters[idx + deleted_chars..].iter() {
      if *original_char != *modified_char {
        modified_step.push(*modified_char);
        deleted_chars += 1;
      } else {
        break;
      }
    }
  }
  // todo: validate the case where original and modified values are are like this: abcdfg abcde

  modified_step
}

fn main() {
  let original = String::from("abcdfg");
  let modified = String::from("abcde");

  println!("Result: {:?}", find_naughty_step(original, modified));
}
