//! Lint rule implementations.
//!
//! Each rule is a free function that inspects a `Document` and returns
//! zero or more `LintDiag` values. Rules never modify the AST.

use std::path::Path;

use crate::error::Level;
use crate::lint::keys::valid_keys;
use crate::lint::LintDiag;
use crate::parse::ast::{Child, Document, Field, Namespace, NodeType, Value};

// -------------------------------------------------------------------- //
// W001 — Boolean true/false instead of live/dark

pub fn w001(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let mut out = Vec::new();
  visit_fields(doc, &mut |f, _ns| {
    if let Value::Bare(v) = &f.value {
      if *v == "true" || *v == "false" {
        out.push(LintDiag {
          code: "W001",
          level: Level::Warning,
          msg: format!(
            "field `{}` uses `{}` — prefer `{}` (AURA boolean convention)",
            f.key,
            v,
            if *v == "true" { "live" } else { "dark" }
          ),
          file: file.to_path_buf(),
          line: f.span.line,
        });
      }
    }
    if let Value::Union(parts) = &f.value {
      for part in parts {
        if let Value::Bare(v) = part {
          if *v == "true" || *v == "false" {
            out.push(LintDiag {
              code: "W001",
              level: Level::Warning,
              msg: format!(
                "field `{}` contains `{}` — prefer `{}`",
                f.key,
                v,
                if *v == "true" { "live" } else { "dark" }
              ),
              file: file.to_path_buf(),
              line: f.span.line,
            });
          }
        }
      }
    }
  });
  out
}

// -------------------------------------------------------------------- //
// W002 — Deprecated keys (thumbnail, artwork, aura-ref)

pub fn w002(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let mut out = Vec::new();
  visit_fields(doc, &mut |f, _ns| {
    let replacement = match f.key {
      "thumbnail" => Some("cover -> @art/id"),
      "artwork" => Some("cover -> @art/id"),
      // aura-ref was renamed to ref in the AURA 0.2 language revision.
      "aura-ref" => Some("ref -> @track/id  (key renamed from `aura-ref` to `ref`)"),
      _ => None,
    };
    if let Some(repl) = replacement {
      out.push(LintDiag {
        code: "W002",
        level: Level::Warning,
        msg: format!("key `{}` is removed — use `{}` instead", f.key, repl),
        file: file.to_path_buf(),
        line: f.span.line,
      });
    }
  });
  out
}

// -------------------------------------------------------------------- //
// W003 — Interval-indexed node missing `time` field

pub fn w003(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let mut out = Vec::new();
  visit_namespaces(doc, &mut |ns| {
    if !ns.node_type.is_interval() {
      return;
    }
    let has_time = ns
      .children
      .iter()
      .any(|c| matches!(c, Child::Field(f) if f.key == "time"));
    if !has_time {
      out.push(LintDiag {
        code: "W003",
        level: Level::Warning,
        msg: format!(
          "node `{}` is interval-indexed but has no `time` field",
          ns.name
        ),
        file: file.to_path_buf(),
        line: ns.span.line,
      });
    }
  });
  out
}

// -------------------------------------------------------------------- //
// W004 — Manifest missing `name`

pub fn w004(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let mut out = Vec::new();
  visit_namespaces(doc, &mut |ns| {
    if !matches!(ns.node_type, NodeType::Manifest | NodeType::Collection) {
      return;
    }
    let has_name = ns
      .children
      .iter()
      .any(|c| matches!(c, Child::Field(f) if f.key == "name"));
    if !has_name {
      out.push(LintDiag {
        code: "W004",
        level: Level::Warning,
        msg: format!("`{}::` block is missing required `name` field", ns.name),
        file: file.to_path_buf(),
        line: ns.span.line,
      });
    }
  });
  out
}

// -------------------------------------------------------------------- //
// W005 — Manifest missing `creator` / `author`

pub fn w005(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let mut out = Vec::new();
  visit_namespaces(doc, &mut |ns| {
    if ns.node_type != NodeType::Manifest {
      return;
    }
    let has_creator = ns
      .children
      .iter()
      .any(|c| matches!(c, Child::Field(f) if f.key == "creator" || f.key == "author"));
    if !has_creator {
      out.push(LintDiag {
        code: "W005",
        level: Level::Warning,
        msg: format!("`manifest::` block is missing `creator` or `author` field"),
        file: file.to_path_buf(),
        line: ns.span.line,
      });
    }
  });
  out
}

// -------------------------------------------------------------------- //
// W006 — Unknown key (strict mode only)
//
// Skipped for blocks with `raw_slug = true` (vocab slug blocks like `$live::`)
// since they use domain-specific field names.

pub fn w006(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  let valid = valid_keys();
  let mut out = Vec::new();
  visit_fields_with_parent(doc, &mut |f, ns| {
    // Skip structural / inherits pseudo-key
    if f.key == ">>" || f.key.is_empty() {
      return;
    }
    // Skip W006 for:
    //  • raw_slug blocks (`$live::`, `$dark::`) — domain-specific field names
    //  • Contains blocks — keys are generated AURA IDs, not vocabulary keys
    if ns.raw_slug || matches!(ns.node_type, NodeType::Contains) {
      return;
    }
    if !valid.contains(f.key) {
      out.push(LintDiag {
        code: "W006",
        level: Level::Warning,
        msg: format!("key `{}` is not in the standard AURA vocabulary", f.key),
        file: file.to_path_buf(),
        line: f.span.line,
      });
    }
  });
  out
}

// -------------------------------------------------------------------- //
// E001 — Required field (!) absent

pub fn e001(doc: &Document<'_>, file: &Path) -> Vec<LintDiag> {
  use crate::parse::ast::FieldMarker;
  let mut out = Vec::new();
  // Collect all required keys per namespace, then check they are present.
  visit_namespaces(doc, &mut |ns| {
    let required: Vec<&str> = ns
      .children
      .iter()
      .filter_map(|c| {
        if let Child::Field(f) = c {
          if f.marker == Some(FieldMarker::Required) {
            return Some(f.key);
          }
        }
        None
      })
      .collect();

    for req_key in required {
      let present = ns
        .children
        .iter()
        .any(|c| matches!(c, Child::Field(f) if f.key == req_key));
      if !present {
        out.push(LintDiag {
          code: "E001",
          level: Level::Error,
          msg: format!(
            "required field `{}` (marked with `!`) is absent from `{}::`",
            req_key, ns.name
          ),
          file: file.to_path_buf(),
          line: ns.span.line,
        });
      }
    }
  });
  out
}

// -------------------------------------------------------------------- //
// AST visitors

fn visit_namespaces<'a, F>(doc: &'a Document<'_>, visitor: &mut F)
where
  F: FnMut(&'a Namespace<'_>),
{
  for ns in &doc.namespaces {
    visit_ns(ns, visitor);
  }
}

fn visit_ns<'a, F>(ns: &'a Namespace<'_>, visitor: &mut F)
where
  F: FnMut(&'a Namespace<'_>),
{
  visitor(ns);
  for child in &ns.children {
    if let Child::Block(b) = child {
      visit_ns(b, visitor);
    }
  }
}

fn visit_fields<'a, F>(doc: &'a Document<'_>, visitor: &mut F)
where
  F: FnMut(&'a Field<'_>, &'a Namespace<'_>),
{
  for ns in &doc.namespaces {
    visit_fields_ns(ns, visitor);
  }
}

fn visit_fields_ns<'a, F>(ns: &'a Namespace<'_>, visitor: &mut F)
where
  F: FnMut(&'a Field<'_>, &'a Namespace<'_>),
{
  for child in &ns.children {
    match child {
      Child::Field(f) => visitor(f, ns),
      Child::Block(b) => visit_fields_ns(b, visitor),
    }
  }
}

/// Visits all fields, passing the immediate parent namespace to the visitor.
/// Used by W006 so it can check the `raw_slug` flag of the parent block.
fn visit_fields_with_parent<'a, F>(doc: &'a Document<'_>, visitor: &mut F)
where
  F: FnMut(&'a Field<'_>, &'a Namespace<'_>),
{
  for ns in &doc.namespaces {
    visit_fields_with_parent_ns(ns, visitor);
  }
}

fn visit_fields_with_parent_ns<'a, F>(ns: &'a Namespace<'_>, visitor: &mut F)
where
  F: FnMut(&'a Field<'_>, &'a Namespace<'_>),
{
  for child in &ns.children {
    match child {
      Child::Field(f) => visitor(f, ns),
      Child::Block(b) => visit_fields_with_parent_ns(b, visitor),
    }
  }
}
