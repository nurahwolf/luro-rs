pub fn extract_help_from_doc_comments(attrs: &[syn::Attribute]) -> (Option<String>, Option<String>) {
    let mut doc_lines = String::new();
    for attr in attrs {
        if let syn::Meta::NameValue(doc_attr) = &attr.meta {
            if doc_attr.path == quote::format_ident!("doc").into() {
                if let syn::Expr::Lit(lit_expr) = &doc_attr.value {
                    if let syn::Lit::Str(literal) = &lit_expr.lit {
                        doc_lines += literal.value().trim(); // Trim lines like rustdoc does
                        doc_lines += "\n";
                    }
                }
            }
        }
    }

    // Trim trailing newline and apply newline escapes
    let doc_lines = doc_lines.trim().replace("\\\n", "");

    let mut paragraphs = doc_lines.splitn(2, "\n\n").filter(|x| !x.is_empty()); // "".split => [""]

    // Pop first paragraph as description if needed (but no newlines bc description is single line)
    let description = paragraphs.next().map(|x| x.replace("\n", " "));
    // Use rest of doc comments as help text
    let help_text = paragraphs.next().map(|x| x.to_owned());

    (description, help_text)
}

pub fn extract_type_parameter<'a>(outer_type: &str, t: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(path) = t {
        if path.path.segments.len() == 1 {
            let path = &path.path.segments[0];
            if path.ident == outer_type {
                if let syn::PathArguments::AngleBracketed(generics) = &path.arguments {
                    if generics.args.len() == 1 {
                        if let syn::GenericArgument::Type(t) = &generics.args[0] {
                            return Some(t);
                        }
                    }
                }
            }
        }
    }
    None
}
