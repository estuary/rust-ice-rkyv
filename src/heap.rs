use crate::bump_str::BumpStr;

#[derive(Debug, rkyv::Archive, rkyv::Serialize)]
#[archive(
    archived = "ArchivedNode",
    bound(
        serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer + rkyv::ser::SharedSerializeRegistry"
    )
)]
pub enum HeapNode<'alloc> {
    String(BumpStr<'alloc>),
}

