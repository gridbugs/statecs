pub const ENTITY_REF: &'static str = r#"
#[derive(Clone, Copy)]
pub struct EntityRef<'a> {
    ecs: &'a EcsCtx,
    id: EntityId,
}

impl<'a> Entity<'a> for EntityRef<'a> {
    type Ecs = EcsCtx;
    fn ecs(self) -> &'a Self::Ecs { self.ecs }
    fn id(self) -> EntityId { self.id }
}
"#;
