use super::{
    TileValue,
    Direction,
};

use std::fmt;
use rand::{
    random,
    thread_rng,
    Rng,
};

#[derive(Debug, Clone, Copy)]
pub struct Board
{
    tiles: [[Option<TileValue>; 4]; 4],
}

impl Board
{
    pub fn new() -> Board
    {
        let mut tiles = [[None; 4]; 4];

        tiles[(random::<u8>() % 4) as usize][(random::<u8>() % 4) as usize] =
            TileValue::new(if thread_rng().gen_bool(0.1) { 4 } else { 2 });

        let (mut x, mut y);

        while {
            x = (random::<u8>() % 4) as usize;
            y = (random::<u8>() % 4) as usize;

            tiles[x][y].is_some()
        } {}

        tiles[x][y] =
            TileValue::new(if thread_rng().gen_bool(0.1) { 4 } else { 2 });

        Board { tiles }
    }

    pub fn count_score(self) -> usize
    {
        self.tiles.iter().map(
            |row| row.iter().map(
                |tile| tile.map(|tile| <TileValue as Into<usize>>::into(tile)).unwrap_or(0_usize)
            ).sum::<usize>()
        ).sum::<usize>()
    }

    pub fn make_move(&mut self, dir: Direction) -> Option<usize>
    {
        let mut move_was_valid = false;
        let mut added_score = 0;

        for index in 0..4
        {
            for depth in (0..3).rev()
            {
                if let Some(to_move) = self.get_cell(dir, depth, index)
                {
                    let mut target_depth = None;

                    for move_depth in (depth + 1)..=3
                    {
                        match self.get_cell(dir, move_depth, index)
                        {
                            None => target_depth = Some(move_depth),
                            Some(target) => {
                                if target == to_move { target_depth = Some(move_depth); }
                                else { break; }
                            }
                        }
                    }

                    if let Some(target_depth) = target_depth
                    {
                        move_was_valid = true;

                        *self.get_mut_cell(dir, depth, index) = None;

                        let target = self.get_mut_cell(dir, target_depth, index);

                        if target.is_none()
                        { *target = Some(to_move); }
                        else
                        {
                            *target = Some(to_move.next_value());
                            added_score += <TileValue as Into<usize>>::into(to_move.next_value());
                        }
                    }
                }
            }
        }

        if !move_was_valid
        {
            return None;
        }

        let mut available_cells: Vec<(usize, usize)> = Vec::new();

        for x in 0..4
        {
            for y in 0..4
            {
                if let None = self.tiles[y][x]
                {
                    available_cells.push((x, y));
                }
            }
        }

        if available_cells.is_empty()
        {
            return Some(added_score);
        }

        let (x, y) = available_cells[random::<usize>() % available_cells.len()];

        self.tiles[y][x] = TileValue::new(if thread_rng().gen_bool(0.1) { 4 } else { 2 });

        Some(added_score)
    }

    pub fn can_make_move(&self) -> bool
    {
        for row in &self.tiles
        {
            for tile_value in row
            {
                if let None = tile_value
                {
                    return true;
                }
            }
        }

        for y in 0..4
        {
            let mut last_value = None;

            for x in 0..4
            {
                if let Some(tile_value) = self.tiles[y][x]
                {
                    if Some(tile_value) == last_value
                    { return true; }
                    else
                    { last_value = Some(tile_value); }
                }
            }
        }

        for x in 0..4
        {
            let mut last_value = None;

            for y in 0..4
            {
                if let Some(tile_value) = self.tiles[y][x]
                {
                    if Some(tile_value) == last_value
                    { return true; }
                    else
                    { last_value = Some(tile_value); }
                }
            }
        }

        false
    }

    fn get_cell(&self, direction: Direction, depth: usize, index: usize) -> Option<TileValue>
    {
        match direction
        {
            Direction::Up    => self.tiles[3 - depth][index],
            Direction::Down  => self.tiles[depth][index],
            Direction::Left  => self.tiles[index][3 - depth],
            Direction::Right => self.tiles[index][depth],
        }
    }

    fn get_mut_cell(&mut self, direction: Direction, depth: usize, index: usize) -> &mut Option<TileValue>
    {
        match direction
        {
            Direction::Up    => &mut self.tiles[3 - depth][index],
            Direction::Down  => &mut self.tiles[depth][index],
            Direction::Left  => &mut self.tiles[index][3 - depth],
            Direction::Right => &mut self.tiles[index][depth],
        }
    }
}

impl fmt::Display for Board
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        let mut largest_value: usize = 0;

        for row in &self.tiles
        {
            for tile_value in row
            {
                if let Some(tile_value) = tile_value
                {
                    largest_value = largest_value.max((*tile_value).into());
                }
            }
        }

        let width = format!("{}", largest_value).len() + 2;

        let spacer = 
            repeat("-")
                .take(3 + width * 4)
                .collect::<Vec<_>>()
                .join("");
        
        use std::iter::repeat;

        write!(fmt, "/{}\\\n", spacer)?;
        for (i, row) in self.tiles.iter().enumerate()
        {
            if i != 0 { write!(fmt, "|{}|\n", spacer)?; }

            if width >= 5
            {
                write!(fmt, "|")?;
                for _ in 0..4 { write!(fmt, "{:width$}|", "", width=width)?; }
                write!(fmt, "\n")?;
            }

            write!(fmt, "|")?;
            for tile in row.iter()
            {
                if let Some(tile_value) = tile
                { write!(fmt, "{:^width$}|", tile_value, width=width)?; }
                else
                { write!(fmt, "{:width$}|", "", width=width)?; }
            }
            write!(fmt, "\n")?;

            if width >= 5
            {
                write!(fmt, "|")?;
                for _ in 0..4 { write!(fmt, "{:width$}|", "", width=width)?; }
                write!(fmt, "\n")?;
            }
        }
        write!(fmt, "\\{}/", spacer)?;

        Ok(())
    }
}
