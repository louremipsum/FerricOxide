pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        2 => format!(
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottle of beer on the wall.\n",
            n,
            n,
            n - 1
        ),
        _ => format!(
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n",
            n,
            n,
            n - 1
        ),
    }
}
pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    let mut i: u32 = start;
    loop {
        s = s + &verse(i);
        if i <= end {
            break;
        }
        s = s + "\n";
        i -= 1;
    }
    s
}
