mod tile_value;
use tile_value::*;

mod board;
use board::*;

mod direction;
use direction::*;

use std::io::{
    Read,
    stdin,
};
use raw_tty::GuardMode;

fn main() -> std::io::Result<()>
{
    let mut stdin = stdin().guard_mode()?;
    stdin.modify_mode(|mut ios| {
        ios.c_lflag &= !libc::ECHO;
        ios.c_lflag &= !libc::ICANON;
        ios
    })?;
    let mut in_bytes = stdin.bytes();

    let mut board = Board::new();
    let mut score = 0;

    println!("{}", board);

    loop
    {
        if let Some(key) = in_bytes.next()
        {
            let key = key?;
            if key == 3 { break; }

            if let Some(direction) = Direction::from_wasd(key as char)
            {
                if let Some(added_score) = board.make_move(direction)
                {
                    score += added_score;

                    println!("Score: {}", score);
                    println!("{}", board);

                    if !board.can_make_move()
                    {
                        println!("Game over");
                        break;
                    }
                }
            }
        }
        else { break; }
    }

    Ok(())
}
