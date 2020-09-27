use super::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type QueryWithEnemy<'a, T> = Query<'a, With<Enemy, T>>;

pub fn enemies_movement_system(
    mut current_turn: ResMut<CurrentTurn>,
    mut enemies_position_query: QueryWithEnemy<(
        &Viewshed,
        &mut RememberedObstacles,
        &mut LastPlayerPosition,
        &mut GameCoordinates,
    )>,
    mut player_position_query: Query<With<Player, &GameCoordinates>>,
    mut obstacles_query: Query<Without<Tile, &GameCoordinates>>,
) {
    if current_turn.side == GameSide::Enemies {
        let player_position = player_position_query.iter().iter().next().unwrap().position;

        let mut obstacles = obstacles_query
            .iter()
            .iter()
            .map(|c| c.position)
            .collect::<HashSet<_>>();

        for (viewshed, mut remembered_obstacles, mut last_player_position, mut enemy_coordinates) in
            &mut enemies_position_query.iter()
        {
            if viewshed.visible_positions.contains(&player_position) {
                last_player_position.0 = Some(player_position);
            }

            remembered_obstacles.0 = remembered_obstacles
                .0
                .difference(&viewshed.visible_positions)
                .cloned()
                .collect();

            let visible_obstacles = obstacles
                .iter()
                .filter(|&obstacle_position| viewshed.visible_positions.contains(obstacle_position))
                .collect::<HashSet<_>>();

            remembered_obstacles.0.extend(visible_obstacles);

            let mut moved = false;

            if let Some(goal) = last_player_position.0 {
                let enemy_position = enemy_coordinates.position;

                if goal == enemy_position {
                    last_player_position.0 = None;
                    continue;
                }

                if let Some(GameCoordinates {
                    position: next_tile,
                    tangent: Some(direction),
                }) = first_step(&enemy_coordinates, &goal, &remembered_obstacles.0)
                {
                    if !obstacles.contains(&next_tile) {
                        obstacles.remove(&enemy_position);
                        turn_and_move(&mut enemy_coordinates, direction);
                        obstacles.insert(enemy_coordinates.position);
                        moved = true;
                    }
                }
            }

            if !moved {
                enemy_coordinates.tangent = Some(
                    enemy_coordinates
                        .tangent
                        .unwrap()
                        .rotate(&enemy_coordinates.position.normal),
                );
            }
        }

        current_turn.side = GameSide::Sun
    }
}

fn first_step(
    start: &GameCoordinates,
    goal: &Position,
    obstacles: &HashSet<Position>,
) -> Option<GameCoordinates> {
    let mut frontier = BinaryHeap::new();
    frontier.push(QueueElement {
        node: PathNode {
            coordinates: *start,
            came_from: start.tangent.unwrap(),
        },
        priority: 0,
    });

    let mut came_from = HashMap::new();

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start.position, 0);

    while let Some(queue_element) = frontier.pop() {
        let current = queue_element.node.coordinates.position;

        if current == *goal {
            break;
        }

        for PathNode {
            coordinates: next,
            came_from: direction,
        } in neighbours(&current)
        {
            if !obstacles.contains(&next.position) || next.position == *goal {
                let new_cost = cost_so_far.get(&current).unwrap() + 1;

                if let Some(&previous_cost) = cost_so_far.get(&next.position) {
                    if previous_cost <= new_cost {
                        continue;
                    }
                }

                cost_so_far.insert(next.position, new_cost);
                let priority = 2 * new_cost + queue_element.heuristic(goal);

                frontier.push(QueueElement {
                    node: PathNode {
                        coordinates: next,
                        came_from: direction,
                    },
                    priority,
                });

                came_from.insert(next.position, (current, direction));
            }
        }
    }

    let mut current = goal;

    while came_from.contains_key(&current) {
        let (previous, direction) = came_from.get(&current).unwrap();

        if *previous == start.position {
            return Some(GameCoordinates {
                position: *current,
                tangent: Some(*direction),
            });
        } else {
            current = previous;
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct QueueElement {
    node: PathNode,
    priority: isize,
}

impl QueueElement {
    fn heuristic(&self, goal: &Position) -> isize {
        2 * self.node.coordinates.position.manhattan_distance_to(goal)
            - self
                .node
                .came_from
                .to_vector()
                .dot(&self.node.coordinates.tangent.unwrap().to_vector())
    }
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then(other.node.cmp(&self.node))
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
