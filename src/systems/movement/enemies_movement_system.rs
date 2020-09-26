use super::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn enemies_movement_system(
    mut current_turn: ResMut<CurrentTurn>,
    mut enemies_position_query: Query<
        With<Enemy, (&Viewshed, &mut LastPlayerPosition, &mut GameCoordinates)>,
    >,
    mut player_position_query: Query<With<Player, &GameCoordinates>>,
    mut obstacles_query: Query<Without<Tile, &GameCoordinates>>,
) {
    if current_turn.side == GameSide::Enemies {
        let player_position = player_position_query
            .iter()
            .iter()
            .next()
            .unwrap()
            .position();

        let mut obstacles = obstacles_query
            .iter()
            .iter()
            .map(|c| (*c).position())
            .collect::<HashSet<_>>();

        for (viewshed, mut last_player_position, mut coordinates) in
            &mut enemies_position_query.iter()
        {
            if viewshed.visible_positions.contains(&player_position) {
                last_player_position.0 = Some(player_position);
            }

            let mut moved = false;

            if let Some(goal) = last_player_position.0 {
                let visible_obstacles = obstacles
                    .iter()
                    .filter(|&obstacle_position| {
                        viewshed.visible_positions.contains(obstacle_position)
                    })
                    .collect::<HashSet<_>>();

                let enemy_position = coordinates.position();

                if goal == enemy_position {
                    last_player_position.0 = None;
                    continue;
                }

                if let Some((next_tile, direction)) =
                    next_tile(&enemy_position, &goal, &visible_obstacles)
                {
                    if !obstacles.contains(&next_tile) {
                        obstacles.remove(&enemy_position);
                        turn_and_move(&mut coordinates, direction);
                        obstacles.insert(coordinates.position());
                        moved = true;
                    }
                }
            }

            if !moved {
                coordinates.tangent =
                    Some(coordinates.tangent.unwrap().rotate(&coordinates.normal));
            }
        }

        current_turn.side = GameSide::Sun
    }
}

fn next_tile(
    start: &Position,
    goal: &Position,
    obstacles: &HashSet<&Position>,
) -> Option<(Position, UnitVector)> {
    let mut frontier = BinaryHeap::new();
    frontier.push(PathNode {
        position: *start,
        priority: 0,
    });

    let mut came_from = HashMap::new();

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(*start, 0);

    while let Some(path_node) = frontier.pop() {
        let current = path_node.position;

        if current == *goal {
            break;
        }

        for (next, direction) in neighbours(&current) {
            if !obstacles.contains(&next) || next == *goal {
                let new_cost = cost_so_far.get(&current).unwrap() + 1;

                let current_cost = cost_so_far.entry(next).or_insert(new_cost + 1);

                if new_cost < *current_cost {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost + path_node.heuristic(goal);

                    frontier.push(PathNode {
                        position: next,
                        priority,
                    });

                    came_from.insert(next, (current, direction));
                }
            }
        }
    }

    let mut current = goal;

    while came_from.contains_key(&current) {
        let (previous, direction) = came_from.get(&current).unwrap();

        if previous == start {
            return Some((*current, *direction));
        } else {
            current = previous;
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PathNode {
    position: Position,
    priority: isize,
}

impl PathNode {
    fn heuristic(&self, goal: &Position) -> isize {
        self.position.manhattan_distance_to(goal)
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = other.priority.cmp(&self.priority);

        match ordering {
            Ordering::Equal => {
                let self_vector = self.position.cubelet + self.position.normal.to_vector();
                let other_vector = other.position.cubelet + other.position.normal.to_vector();

                self_vector
                    .x
                    .cmp(&other_vector.x)
                    .then(self_vector.y.cmp(&other_vector.y))
                    .then(self_vector.z.cmp(&other_vector.z))
            }
            _ => ordering,
        }
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
