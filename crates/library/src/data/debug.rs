use super::{super::store::*, id::*, metadata::*, property::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, io},
};

/// Write debug metadata.
pub fn write_debug_metadata<WriteT>(
    metadata: &Metadata,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::write_debug_field("metadata", last, writer, context, |writer, context| -> io::Result<()> {
        metadata.write_debug_for(writer, context)
    })
}

/// Write debug ID.
pub fn write_debug_id<WriteT>(
    name: &str,
    id: Option<&ID>,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    context.indent_into_branch(writer, last)?;
    context.theme.write_meta(writer, name)?;
    context.theme.write_delimiter(writer, ":")?;
    match id {
        Some(id) => id.write_debug_for(writer, context),
        None => {
            context.separate(writer)?;
            context.theme.write_bare(writer, "None")
        }
    }
}

/// Write debug properties.
pub fn write_debug_properties<WriteT>(
    name: &str,
    properties: &BTreeMap<String, Property>,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::write_debug_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        if properties.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "{}")?;
        } else {
            for ((name, property), last) in IterateWithLast::new(properties) {
                context.indent_into_branch(writer, last)?;
                context.theme.write_meta(writer, name)?;
                context.theme.write_delimiter(writer, ":")?;
                property
                    .write_debug_for(writer, &context.child().with_inline(true).increase_indentation_branch(last))?;
            }
        }

        Ok(())
    })
}

/// Write debug types.
pub fn write_debug_types<WriteT, StoreT>(
    type_ids: &Vec<ID>,
    store: &StoreT,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
    StoreT: StoreClient,
{
    context.indent_into_branch(writer, false)?;
    context.theme.write_meta(writer, "types")?;
    context.theme.write_delimiter(writer, ":")?;

    if type_ids.is_empty() {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, "[]")?;
    } else {
        let item_context = context.child().increase_indentation_branch(false);
        for (type_id, last) in IterateWithLast::new(type_ids) {
            item_context.indent_into_double_branch(writer, last)?;
            match store.get_type(type_id).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                Some(type_) => {
                    type_
                        .to_debuggable(store)
                        .write_debug_for(writer, &item_context.child().increase_indentation_double_branch(last))?;
                }

                None => {
                    write!(writer, "{}: {}", context.theme.meta("type_id"), type_id)?;
                }
            }
        }
    }

    Ok(())
}
