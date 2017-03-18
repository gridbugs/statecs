pub const ECS_POST_ACTION: &'static str = r#"

pub struct EcsPostAction<'a, 'b> {
    ecs: &'a EcsCtx,
    action: &'b EcsAction,
}

"#;
