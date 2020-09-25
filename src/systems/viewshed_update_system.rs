use crate::components::*;
use bevy::prelude::*;
use num_rational::Rational;
use std::collections::{HashSet, VecDeque};

pub fn viewshed_update_system(
    mut viewer_coordinates_query: Query<(&mut Viewshed, &GameCoordinates)>,
    mut obstacles_query: Query<(&Opaque, &GameCoordinates)>,
) {
    for (mut viewshed, viewer_coordinates) in &mut viewer_coordinates_query.iter() {
        viewshed.visible_positions.clear();
        let viewer_position = viewer_coordinates.position();

        viewshed.visible_positions.insert(viewer_position);

        let obstacles = obstacles_query
            .iter()
            .iter()
            .map(|t| (*t.1).position())
            .collect::<HashSet<_>>();

        use Cardinal::*;

        let cardinals = match viewshed.shape {
            ViewshedShape::Circle => vec![North, East, South, West],
            ViewshedShape::Quadrant => {
                if let Some(cardinal) = view_direction(viewer_coordinates) {
                    vec![cardinal]
                } else {
                    vec![]
                }
            }
        };

        for &cardinal in cardinals.iter() {
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
                            viewshed.visible_positions.insert(position);
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

fn view_direction(coordinates: &GameCoordinates) -> Option<Cardinal> {
    let (abscissa, ordinate) = coordinates.normal.abscissa_and_ordinate();

    use Cardinal::*;

    if let Some(tangent) = coordinates.tangent {
        if tangent == ordinate {
            Some(North)
        } else if tangent == -ordinate {
            Some(South)
        } else if tangent == abscissa {
            Some(East)
        } else if tangent == -abscissa {
            Some(West)
        } else {
            None
        }
    } else {
        None
    }
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

        let mut y_shift = row * ordinate;

        let mut flipped = false;

        let mut y_position = Position {
            cubelet: self.origin.cubelet + y_shift,
            normal: self.origin.normal,
        };

        if !y_position.on_surface() {
            y_shift = (row - 1) * (-self.origin.normal);
            y_position.cubelet = self.origin.cubelet + y_shift;
            y_position.normal = ordinate;

            flipped = true;
        }

        if !y_position.on_surface() {
            return None;
        }

        let mut x_shift = col * abscissa;

        let mut position = Position {
            cubelet: y_position.cubelet + x_shift,
            normal: y_position.normal,
        };

        if !position.on_surface() && !flipped {
            x_shift = (col - 1) * (-self.origin.normal);
            position.cubelet = y_position.cubelet + x_shift;
            position.normal = abscissa;

            if !position.on_surface() {
                x_shift = (col + 1) * self.origin.normal;
                position.cubelet = y_position.cubelet + x_shift;
                position.normal = -abscissa;
            }
        }

        if !position.on_surface() {
            None
        } else {
            Some(position)
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
