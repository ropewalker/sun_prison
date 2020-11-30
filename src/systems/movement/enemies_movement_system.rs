use super::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type QueryWithEnemy<'a, T> = Query<'a, With<Enemy, T>>;
type EnemyComponents<'a> = (
    &'a Viewshed,
    &'a mut RememberedObstacles,
    &'a mut LastPlayerPosition,
    &'a mut GameCoordinates,
);
type QueryWithObstacle<'a, T> = Query<'a, With<Obstacle, T>>;

pub fn enemies_movement_system(
    mut game_state: ResMut<GameState>,
    mut turn_queue: ResMut<TurnQueue>,
    mut player_position_query: Query<With<Player, (&mut Health, &GameCoordinates)>>,
    mut label_query: Query<(&mut Text, &Label)>,
    mut queries: QuerySet<(
        QueryWithEnemy<EnemyComponents>,
        QueryWithObstacle<&GameCoordinates>,
    )>,
) {
    if let GameState::EnemyTurn(enemy) = *game_state {
        let player = player_position_query.iter_mut().next().unwrap();
        let mut player_health = player.0;
        let player_position = player.1.position;

        let mut obstacles = queries
            .q1_mut()
            .iter()
            .map(|c| c.position)
            .collect::<HashSet<_>>();

        let mut hits = 0;

        let (viewshed, mut remembered_obstacles, mut last_player_position, mut enemy_coordinates) =
            queries.q0_mut().get_mut(enemy).unwrap();

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
        let mut queue_element = (*turn_queue).0.peek_mut().unwrap();

        if let Some(goal) = last_player_position.0 {
            let enemy_position = enemy_coordinates.position;

            if goal == enemy_position {
                last_player_position.0 = None;
            } else if let Some(GameCoordinates {
                position: next_tile,
                tangent: Some(direction),
            }) = first_step(&enemy_coordinates, &goal, &remembered_obstacles.0)
            {
                if !obstacles.contains(&next_tile) {
                    obstacles.remove(&enemy_position);
                    turn_and_move(&mut enemy_coordinates, direction);
                    obstacles.insert(enemy_coordinates.position);
                } else {
                    enemy_coordinates.tangent = Some(direction);

                    if player_position == next_tile {
                        player_health.0 -= 1;
                        hits += 1;
                    }
                }

                (*queue_element).priority += MOVE_COST;
                moved = true;
            }
        }

        if !moved {
            enemy_coordinates.tangent = Some(
                enemy_coordinates
                    .tangent
                    .unwrap()
                    .rotate(&enemy_coordinates.position.normal),
            );
            (*queue_element).priority += ROTATE_COST;
        }

        for (mut text, label) in label_query.iter_mut() {
            if label.label_type == LabelType::GameEvents {
                if hits > 0 {
                    (*text).value = format!("You got hit for {} HP.", hits);
                } else {
                    (*text).value = "It is dark here.".to_string();
                }
            }
        }

        if player_health.0 <= 0 {
            *game_state = GameState::Defeat;
        } else {
            *game_state = GameState::PassingTurn;
        }
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
