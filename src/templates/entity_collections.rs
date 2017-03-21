pub const ENTITY_COLLECTIONS: &'static str = r#"

{{#if ecs_ctx_hash_collections}}

type EcsCtxEntitySet = EntityHashSet;
type EcsCtxEntityMap<T> = EntityHashMap<T>;
type EcsCtxEntitySetIter<'a> = EntityHashSetIter<'a>;
type EcsCtxEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;
type EcsCtxEntityMapCopyIter<'a, T> = EntityHashMapCopyIter<'a, T>;
type EcsCtxEntityMapKeys<'a, T> = EntityHashMapKeys<'a, T>;

{{else}}

type EcsCtxEntitySet = EntityBTreeSet;
type EcsCtxEntityMap<T> = EntityBTreeMap<T>;
type EcsCtxEntitySetIter<'a> = EntityBTreeSetIter<'a>;
type EcsCtxEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;
type EcsCtxEntityMapCopyIter<'a, T> = EntityBTreeMapCopyIter<'a, T>;
type EcsCtxEntityMapKeys<'a, T> = EntityBTreeMapKeys<'a, T>;
    {{#if combine_flag_set}}
type EcsCtxEntitySetRange<'a> = EntityBTreeSetRange<'a>;
    {{/if}}
{{/if}}

{{#if ecs_action_hash_collections}}

type EcsActionEntitySet = EntityHashSet;
type EcsActionEntityMap<T> = EntityHashMap<T>;
type EcsActionEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;

{{else}}

type EcsActionEntitySet = EntityBTreeSet;
type EcsActionEntityMap<T> = EntityBTreeMap<T>;
type EcsActionEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;

{{/if}}

{{#if combine_flag_set}}
pub type FlagIdIter<'a> = EcsCtxEntitySetRange<'a>;
{{else}}
pub type FlagIdIter<'a> = EcsCtxEntitySetIter<'a>;
{{/if}}

pub type EntitySet = EcsCtxEntitySet;
pub type EntityMap<T> = EcsCtxEntityMap<T>;

"#;
