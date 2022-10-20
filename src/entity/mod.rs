use crate::{Query, Res, ResMut, Time, Timer, With};
use crate::component::{Player, Position};

pub struct GreetTimer(pub Timer);

pub fn greet_player(
    query: Query<&Position, With<Player>>,
    mut timer: ResMut<GreetTimer>,
    time: Res<Time>,
) {
    // 用上次更新后经过的时间更新计时器
    // 如果这导致计时器结束，我们就向大家问好
    // time 上的 delta 字段给出了自上次更新以来经过的时间
    if timer.0.tick(time.delta()).just_finished() {
        for pos in query.iter() {
            println!("x: {}, y: {}", pos.x, pos.y);
        }
    }
}
