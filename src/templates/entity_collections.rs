pub const ENTITY_COLLECTIONS: &'static str = r#"

{{#if ecs_ctx_hash_collections}}

pub type EcsCtxEntitySet = EntityHashSet;
pub type EcsCtxEntityMap<T> = EntityHashMap<T>;
pub type EcsCtxEntitySetIter<'a> = EntityHashSetIter<'a>;
pub type EcsCtxEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;
pub type EcsCtxEntityMapCopyIter<'a, T> = EntityHashMapCopyIter<'a, T>;
pub type EcsCtxEntityMapKeys<'a, T> = EntityHashMapKeys<'a, T>;

pub type EcsCtxFlagIdIter<'a> = EcsCtxEntitySetIter<'a>;

{{else}}

pub type EcsCtxEntitySet = EntityBTreeSet;
pub type EcsCtxEntityMap<T> = EntityBTreeMap<T>;
pub type EcsCtxEntitySetIter<'a> = EntityBTreeSetIter<'a>;
pub type EcsCtxEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;
pub type EcsCtxEntityMapCopyIter<'a, T> = EntityBTreeMapCopyIter<'a, T>;
pub type EcsCtxEntityMapKeys<'a, T> = EntityBTreeMapKeys<'a, T>;

    {{#if combine_flag_set}}
pub type EcsCtxEntitySetRange<'a> = EntityBTreeSetRange<'a>;
pub type EcsCtxFlagIdIter<'a> = EcsCtxEntitySetRange<'a>;
    {{else}}
pub type EcsCtxFlagIdIter<'a> = EcsCtxEntitySetIter<'a>;
    {{/if}}

{{/if}}

{{#if ecs_action_hash_collections}}

pub type EcsActionEntitySet = EntityHashSet;
pub type EcsActionEntityMap<T> = EntityHashMap<T>;
pub type EcsActionEntitySetIter<'a> = EntityHashSetIter<'a>;
pub type EcsActionEntityMapIter<'a, T> = EntityHashMapIter<'a, T>;
pub type EcsActionEntityMapCopyIter<'a, T> = EntityHashMapCopyIter<'a, T>;
pub type EcsActionEntityMapKeys<'a, T> = EntityHashMapKeys<'a, T>;

pub type EcsActionFlagIdIter<'a> = EcsActionEntitySetIter<'a>;

{{else}}

pub type EcsActionEntitySet = EntityBTreeSet;
pub type EcsActionEntityMap<T> = EntityBTreeMap<T>;
pub type EcsActionEntitySetIter<'a> = EntityBTreeSetIter<'a>;
pub type EcsActionEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;
pub type EcsActionEntityMapCopyIter<'a, T> = EntityBTreeMapCopyIter<'a, T>;
pub type EcsActionEntityMapKeys<'a, T> = EntityBTreeMapKeys<'a, T>;

    {{#if combine_flag_set}}
pub type EcsActionEntitySetRange<'a> = EntityBTreeSetRange<'a>;
pub type EcsActionFlagIdIter<'a> = EcsActionEntitySetRange<'a>;
    {{else}}
pub type EcsActionFlagIdIter<'a> = EcsActionEntitySetIter<'a>;
    {{/if}}

{{/if}}

pub type EntitySet = EcsCtxEntitySet;
pub type EntityMap<T> = EcsCtxEntityMap<T>;
pub type EntitySetIter<'a> = EcsCtxEntitySetIter<'a>;
pub type EntityMapIter<'a, T> = EcsCtxEntityMapIter<'a, T>;
pub type EntityMapCopyIter<'a, T> = EcsCtxEntityMapCopyIter<'a, T>;
pub type EntityMapKeys<'a, T> = EcsCtxEntityMapKeys<'a, T>;
"#;
