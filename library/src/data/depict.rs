use super::{super::store::*, id::*, metadata::*, property::*};

use {
    kutil::{
        cli::depict::*,
        std::{iter::*, immutable::*},
    },
    std::{collections::*, io},
};

/// Depict metadata.
pub fn depict_metadata<WriteT>(
    metadata: &Metadata,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::depict_field("metadata", last, writer, context, |writer, context| -> io::Result<()> {
        metadata.depict(writer, context)
    })
}

/// Depict ID.
pub fn depict_id<WriteT>(
    name: &str,
    id: Option<&ID>,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::depict_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        match id {
            Some(id) => id.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        }
    })
}

/// Depict properties.
pub fn depict_properties<'own, StoreT, WriteT>(
    name: &str,
    properties: &BTreeMap<ByteString, Property>,
    store: &'own StoreT,
    last: bool,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    StoreT: Store,
    WriteT: io::Write,
{
    utils::depict_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        if properties.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "{}")?;
        } else {
            for ((name, property), last) in IterateWithLast::new(properties) {
                context.indent_into_branch(writer, last)?;
                context.theme.write_meta(writer, name)?;
                context.theme.write_delimiter(writer, ':')?;
                property.to_depict(store).depict(writer, &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    })
}

/// Depict classes.
pub fn depict_classes<WriteT, StoreT>(
    class_ids: &Vec<ID>,
    store: &StoreT,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    WriteT: io::Write,
    StoreT: Store,
{
    utils::depict_field("classes", false, writer, context, |writer, context| -> io::Result<()> {
        if class_ids.is_empty() {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "[]")?;
        } else {
            for (class_id, last) in IterateWithLast::new(class_ids) {
                context.indent_into_double_branch(writer, last)?;
                match store.get_class(class_id).map_err(io::Error::other)? {
                    Some(class) => {
                        class
                            .to_depict(store)
                            .depict(writer, &context.child().increase_indentation_double_branch(last))?;
                    }

                    None => {
                        class_id.depict(writer, &context.child().with_separator(false))?;
                    }
                }
            }
        }

        Ok(())
    })
}
