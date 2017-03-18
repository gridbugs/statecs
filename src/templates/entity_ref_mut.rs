pub const ENTITY_REF_MUT: &'static str = r#"
pub struct EntityRefMut<'a> {
    ecs: &'a mut EcsCtx,
    id: EntityId,
}

impl<'a> EntityMut for EntityRefMut<'a> {
    type Ecs = EcsCtx;

    fn ecs(&self) -> &Self::Ecs { self.ecs }
    fn ecs_mut(&mut self) -> &mut Self::Ecs { self.ecs }
    fn id(&self) -> EntityId { self.id }
}
"#;
