//! Content access level — the `@access/` governance domain.
//!
//! Access levels form an ordered hierarchy for cascade resolution:
//!
//! ```text
//! open < archived < restricted < gated < embargoed < locked
//! ```
//!
//! A parent collection's access level applies to all members unless
//! explicitly overridden. A member may only restrict further, never
//! relax, without an explicit override.
//!
//! Access nodes compile to ATOM `AccessNode` objects (class `0x13`).
//! The engine evaluates the access bitmask at query time — gated and
//! embargoed statuses are re-evaluated on every request, never baked
//! into the compiled artifact.

/// Content visibility and permission level.
///
/// Referenced in AURA as `@access/open`, `@access/locked`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AccessLevel {
  /// Public — unrestricted, no authentication required.
  Open = 0x01,
  /// Accessible but retired — marked for historical access.
  Archived = 0x02,
  /// Geo- or rights-restricted — available in named territories only.
  Restricted = 0x03,
  /// Conditional — requires subscription, payment, or role.
  Gated = 0x04,
  /// Time-locked — transitions to `Open` after an embargo date.
  Embargoed = 0x05,
  /// Private — authentication required, owner-only.
  Locked = 0x06,
}

impl AccessLevel {
  /// Returns the AURA reference string, e.g. `"open"`.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Open => "open",
      Self::Archived => "archived",
      Self::Restricted => "restricted",
      Self::Gated => "gated",
      Self::Embargoed => "embargoed",
      Self::Locked => "locked",
    }
  }

  /// Parses an access level from an AURA reference string.
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "open" => Some(Self::Open),
      "archived" => Some(Self::Archived),
      "restricted" => Some(Self::Restricted),
      "gated" => Some(Self::Gated),
      "embargoed" => Some(Self::Embargoed),
      "locked" => Some(Self::Locked),
      _ => None,
    }
  }

  /// Returns `true` if `other` is at least as restrictive as `self`.
  /// Used to validate that member overrides never relax a parent's level.
  pub fn at_least_as_restrictive(self, other: Self) -> bool {
    other >= self
  }
}

impl std::fmt::Display for AccessLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}
