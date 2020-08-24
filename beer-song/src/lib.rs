pub fn verse(no_of_bottles: i32) -> String {
    if no_of_bottles == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if no_of_bottles == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if no_of_bottles == 2 {
        format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n", no_of_bottles, no_of_bottles - 1 )
    } else {
        format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", no_of_bottles, no_of_bottles - 1 )
    }
}

pub fn sing(no_of_start_bottles: i32, no_of_end_bottles: i32) -> String {
    let mut song = String::from("");
    let mut count: i32 = no_of_start_bottles;
    while count >= no_of_end_bottles {
        song.push_str(verse(count).as_mut_str());
        song.push_str("\n");
        count = count - 1;
    }
    song.pop();
    song.to_string()
}
