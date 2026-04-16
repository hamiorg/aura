Name:           aura
Version:        0.1.0
Release:        1%{?dist}
Summary:        AURA toolchain CLI
License:        MIT
URL:            https://hami.aduki.org
# BuildArch is set per-target by cargo-generate-rpm at package time.
# For x86_64 builds: BuildArch: x86_64
# For arm64 builds:  BuildArch: aarch64

%description
Compile .aura source files into .atom interval trees, .hami B-Tree
manifests, and .atlas DTW alignment files. Manage project history
with an append-only take/mark/stream system. Scaffold new AURA
projects with aura init.

Part of the Triverse / Hami ecosystem for structured media metadata.

%prep
# No source unpacking needed — binary is pre-compiled by cargo-generate-rpm.

%install
# cargo-generate-rpm handles installation from [package.metadata.generate-rpm]
# in compiler/Cargo.toml.

%files
%attr(755, root, root) /usr/bin/aura

%changelog
* Thu Apr 16 2026 Aduki <hello@aduki.org> - 0.1.0-1
- Initial release
