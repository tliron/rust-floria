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
    utils::write_debug_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        match id {
            Some(id) => id.write_debug_for(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_bare(writer, "None")
            }
        }
    })
}

/// Write debug properties.
pub fn write_debug_properties<'own, StoreT, WriteT>(
    name: &str,
    properties: &BTreeMap<String, Property>,
    store: &'own StoreT,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    StoreT: StoreClient,
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
                    .to_debuggable(store)
                    .write_debug_for(writer, &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    })
}

/// Write debug groups.
pub fn write_debug_groups<WriteT, StoreT>(
    group_ids: &Vec<ID>,
    store: &StoreT,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
    StoreT: StoreClient,
{
    utils::write_debug_field("groups", false, writer, context, |writer, context| -> io::Result<()> {
        if group_ids.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "[]")?;
        } else {
            for (group_id, last) in IterateWithLast::new(group_ids) {
                context.indent_into_double_branch(writer, last)?;
                match store.get_group(group_id).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                    Some(group) => {
                        group
                            .to_debuggable(store)
                            .write_debug_for(writer, &context.child().increase_indentation_double_branch(last))?;
                    }

                    None => {
                        group_id.write_debug_for(writer, &context.child().with_separator(false))?;
                    }
                }
            }
        }

        Ok(())
    })
}
