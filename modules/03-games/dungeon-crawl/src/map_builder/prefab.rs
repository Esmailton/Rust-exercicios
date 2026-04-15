use crate::prelude::*;

const FORTRESS : (&str, i32, i32) = ("
    ------------
    ---######---
    ---#----#---
    ---#-M--#---
    -###----###-
    --M------M--
    -###----###-
    ---#----#---
    ---#----#---
    ---######---
    ------------
    ", 12, 11);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator){
    let mut placement = None;
    let dijkstra_map = DijstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = React::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2
        );
        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                can_place = True
            }
        });
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spaws.retain(|pt| !points.contains(pt));
        }
        attempts += 1;
    }
}