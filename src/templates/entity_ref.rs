pub const ENTITY_REF: &'static str = r#"
#[derive(Clone, Copy)]
pub struct EntityRef<'a> {
    ecs: &'a EcsCtx,
    id: EntityId,
}

impl<'a> EntityRef<'a> {
    pub fn post<'b>(self, action: &'b EcsAction) -> EntityRefPostAction<'a, 'b> {
        self.ecs.post_entity(action, self.id)
    }
    pub fn new(ecs: &'a EcsCtx, id: EntityId) -> Self {
        EntityRef {
            ecs: ecs,
            id: id,
        }
    }
}

impl<'a> Entity<'a> for EntityRef<'a> {
    type Ecs = EcsCtx;
    fn ecs(self) -> &'a Self::Ecs { self.ecs }
    fn id(self) -> EntityId { self.id }
}
"#;
