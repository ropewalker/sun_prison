use crate::components::*;
use bevy::prelude::*;
use num_rational::Rational;
use std::collections::{HashSet, VecDeque};

pub fn viewshed_update_system(
    mut viewer_coordinates_query: Query<(&mut Viewshed, &GameCoordinates)>,
    mut obstacles_query: Query<(&Opaque, &GameCoordinates)>,
) {
    for (mut viewshed, viewer_coordinates) in &mut viewer_coordinates_query.iter() {
        viewshed.0.clear();
        let viewer_position = Position {
            cubelet: viewer_coordinates.cubelet,
            normal: viewer_coordinates.normal,
        };

        viewshed.0.insert(viewer_position);

        let obstacles: HashSet<Position> = obstacles_query
            .iter()
            .iter()
            .map(|t| (*t.1).into())
            .collect::<HashSet<_>>();

        use Cardinal::*;

        for &cardinal in [North, East, South, West].iter() {
            let quadrant = Quadrant {
                cardinal,
                origin: viewer_position,
            };

            let mut rows = VecDeque::new();

            rows.push_back(Row {
                depth: 1,
                start_slope: Rational::from_integer(-1),
                end_slope: Rational::from_integer(1),
            });

            while !rows.is_empty() {
                let mut row = rows.pop_front().unwrap();
                let mut prev_tile: Option<Tile> = None;

                for tile in row.tiles() {
                    if is_wall(&Some(tile), &quadrant, &obstacles) || is_symmetric(&row, &tile) {
                        if let Some(position) = quadrant.transform(&tile) {
                            viewshed.0.insert(position);
                        }
                    }

                    if is_wall(&prev_tile, &quadrant, &obstacles)
                        && is_floor(&Some(tile), &quadrant, &obstacles)
                    {
                        row.start_slope = slope(&tile);
                    }

                    if is_floor(&prev_tile, &quadrant, &obstacles)
                        && is_wall(&Some(tile), &quadrant, &obstacles)
                    {
                        let mut next_row = row.next();
                        next_row.end_slope = slope(&tile);
                        rows.push_back(next_row);
                    }

                    prev_tile = Some(tile);
                }

                if is_floor(&prev_tile, &quadrant, &obstacles) {
                    rows.push_back(row.next());
                }
            }
        }
    }
}

fn is_wall(tile: &Option<Tile>, quadrant: &Quadrant, obstacles: &HashSet<Position>) -> bool {
    if let Some(tile) = tile {
        if let Some(position) = quadrant.transform(tile) {
            obstacles.contains(&position)
        } else {
            true
        }
    } else {
        false
    }
}

fn is_floor(tile: &Option<Tile>, quadrant: &Quadrant, obstacles: &HashSet<Position>) -> bool {
    if let Some(tile) = tile {
        if let Some(position) = quadrant.transform(tile) {
            !obstacles.contains(&position)
        } else {
            false
        }
    } else {
        false
    }
}

#[derive(Copy, Clone)]
enum Cardinal {
    North,
    South,
    West,
    East,
}

struct Quadrant {
    cardinal: Cardinal,
    origin: Position,
}

impl Quadrant {
    fn transform(&self, tile: &Tile) -> Option<Position> {
        use Cardinal::*;

        let (abscissa, ordinate) = self.origin.normal.abscissa_and_ordinate();
        let (abscissa, ordinate) = match self.cardinal {
            North => (abscissa, ordinate),
            South => (-abscissa, -ordinate),
            West => (ordinate, -abscissa),
            East => (-ordinate, abscissa),
        };

        let (row, col) = (tile.row, tile.col);

        let mut normal = self.origin.normal;
        let mut y_shift = row * ordinate;
        let mut x_shift = col * abscissa;

        let mut y_position = Position {
            cubelet: self.origin.cubelet + y_shift,
            normal: self.origin.normal,
        };

        if !y_position.on_surface() {
            y_shift = (row - 1) * (-self.origin.normal);
            y_position.cubelet = self.origin.cubelet + y_shift;
            y_position.normal = ordinate;
            normal = ordinate;
        }

        if !y_position.on_surface() {
            return None;
        }

        let mut x_position = Position {
            cubelet: self.origin.cubelet + x_shift,
            normal: self.origin.normal,
        };

        if !x_position.on_surface() {
            x_shift = (col - 1) * (-self.origin.normal);
            x_position.cubelet = self.origin.cubelet + x_shift;
            x_position.normal = abscissa;
            normal = abscissa;
        }

        if !x_position.on_surface() {
            x_shift = (col + 1) * self.origin.normal;
            x_position.cubelet = self.origin.cubelet + x_shift;
            x_position.normal = -abscissa;
            normal = -abscissa;
        }

        if !x_position.on_surface() {
            return None;
        }

        let position = Position {
            cubelet: self.origin.cubelet + x_shift + y_shift,
            normal,
        };

        if position.on_surface() {
            Some(position)
        } else {
            None
        }
    }
}

fn round_ties_up(n: Rational) -> isize {
    (n + Rational::new_raw(1, 2)).floor().to_integer()
}

fn round_ties_down(n: Rational) -> isize {
    (n - Rational::new_raw(1, 2)).ceil().to_integer()
}

fn slope(tile: &Tile) -> Rational {
    let (row_depth, col) = (tile.row, tile.col);
    Rational::new(2 * col - 1, 2 * row_depth)
}

fn is_symmetric(row: &Row, tile: &Tile) -> bool {
    let col = tile.col;

    Rational::from_integer(col) >= Rational::from_integer(row.depth) * row.start_slope
        && Rational::from_integer(col) <= Rational::from_integer(row.depth) * row.end_slope
}

#[derive(Copy, Clone)]
struct Tile {
    row: isize,
    col: isize,
}

struct Row {
    depth: isize,
    start_slope: Rational,
    end_slope: Rational,
}

impl Row {
    fn tiles(&self) -> impl Iterator<Item = Tile> {
        let min_col = round_ties_up(Rational::from_integer(self.depth) * self.start_slope);
        let max_col = round_ties_down(Rational::from_integer(self.depth) * self.end_slope);

        let depth = self.depth;

        (min_col..=max_col).map(move |col| Tile { row: depth, col })
    }

    fn next(&self) -> Row {
        Row {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }
}
