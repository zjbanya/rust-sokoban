use hecs::Entity;

#[derive(Debug)]
pub struct EntityMoved {
    pub entity: Entity,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    // 当玩家撞上墙壁等障碍物时触发
    PlayerHitObstacle,
    // 当实体被移动时触发
    EntityMoved(EntityMoved),
    // 当箱子被放置在某处时触发
    BoxPlacedSpot(BoxPlacedOnSpot),
}
