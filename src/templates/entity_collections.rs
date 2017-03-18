pub const ENTITY_COLLECTIONS: &'static str = r#"

type EcsCtxEntitySet = EntityBTreeSet;
type EcsCtxEntityMap<T> = EntityBTreeMap<T>;
type EcsCtxEntitySetIter<'a> = EntityBTreeSetIter<'a>;
type EcsCtxEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;
type EcsCtxEntityMapCopyIter<'a, T> = EntityBTreeMapCopyIter<'a, T>;
type EcsCtxEntityMapKeys<'a, T> = EntityBTreeMapKeys<'a, T>;
{{#if combine_flag_set}}
type EcsCtxEntitySetRange<'a> = EntityBTreeSetRange<'a>;
{{/if}}

{{#if combine_flag_set}}
pub type FlagIdIter<'a> = EcsCtxEntitySetRange<'a>;
{{else}}
pub type FlagIdIter<'a> = EcsCtxEntitySetIter<'a>;
{{/if}}

type EcsActionEntitySet = EntityBTreeSet;
type EcsActionEntityMap<T> = EntityBTreeMap<T>;
type EcsActionEntityMapIter<'a, T> = EntityBTreeMapIter<'a, T>;

"#;
