pub const ENTITY_REF_MUT: &'static str = r#"
pub struct EntityRefMut<'a> {
    ecs: &'a mut EcsCtx,
    id: EntityId,
}

impl<'a> EntityMut for EntityRefMut<'a> {
    type EcsMut = EcsCtx;
    fn ecs_mut(&mut self) -> &mut Self::EcsMut { self.ecs }
}

impl<'a> Entity for EntityRefMut<'a> {
    type Ecs = EcsCtx;
    fn ecs(&self) -> &Self::Ecs { self.ecs }
    fn id(&self) -> EntityId { self.id }
}

impl<'a> EntityRefMut<'a> {
    pub fn new(ecs: &'a mut EcsCtx, id: EntityId) -> Self {
        EntityRefMut {
            ecs: ecs,
            id: id,
        }
    }
}
"#;
