pub const COMMIT_INSERTIONS: &'static str = r#"
fn commit_map_insertion<T>(ctx: &mut EcsCtxEntityMap<T>, action: &mut EcsActionEntityMap<T>) {
{{#if ecs_action_hash_collections}}
    for (id, value) in action.drain() {
        ctx.insert(id, value);
    }
{{else}}
    {{#if ecs_ctx_hash_collections}}
    unimplemented!()
    {{else}}
    ctx.append(action);
    {{/if}}
{{/if}}
}

fn commit_set_insertion(ctx: &mut EcsCtxEntitySet, action: &mut EcsActionEntitySet) {
{{#if ecs_action_hash_collections}}
    for id in action.drain() {
        ctx.insert(id);
    }
{{else}}
    {{#if ecs_ctx_hash_collections}}
    unimplemented!()
    {{else}}
    ctx.append(action);
    {{/if}}
{{/if}}
}
"#;
