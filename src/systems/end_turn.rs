use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(MacGuffin)]

pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut muffin = <&Point>::query().filter(component::<MacGuffin>());
    let current_state = turn_state.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let muffin_default = Point::new(-1, -1);
    let muffin_pos = muffin.iter(ecs).nth(0).unwrap_or(&muffin_default);

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == muffin_pos {
            new_state = TurnState::Victory;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}
