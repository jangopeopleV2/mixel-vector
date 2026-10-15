# Asset attribution

Every non-code asset in this repository (icons, images, fonts, example art, colour profiles, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.

**Policy (mandatory):** VectorCraft contains **no Adobe iconography, images, artwork, presets, swatch/brush/symbol/pattern libraries or ICC profiles**. Every asset is either original work by VectorCraft contributors, or third-party material under an open licence (OSI open source, public domain / CC0, or Creative Commons that allows redistribution), and it is attributed below. Screenshots of Adobe software are never committed. The existing upstream brand exception is the ArtCraft name, wordmark and logos in `docs/brand/`: ArtCraft trademarks, not open source, usable only under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).

Generated-in-code art is original and has no file to list. This covers the default swatches, brushes, symbols, patterns, graphic styles, image-trace presets and the vector tool cursors (`crates/ui-egui/src/cursors.rs`).

**Font files are never committed here.** Fonts shared by the Crafting Apps live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts) (rules: craftrules [`standards/fonts.md`](https://github.com/storytold/craftrules/blob/main/standards/fonts.md)). The small Latin UI/document fonts below predate that rule and stay.

**Optional build input: craft-fonts.** Builds made with `CRAFT_FONTS_DIR=<craft-fonts checkout>` (all official releases) embed the Japanese fonts listed in its [`ATTRIBUTION.md`](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md): BIZ UDPGothic Regular and Bold, Shippori Mincho Regular and BIZ UDMincho Regular (web builds: BIZ UDPGothic Regular only), all under the SIL Open Font License 1.1. They are not files in this repository; release packages carry each one's `OFL-<family>.txt`. Builds without `CRAFT_FONTS_DIR` embed none of them.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `docs/brand/artcraft-logo-white.png` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-logo-white.svg` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-logo.png` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-logo.svg` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-mark-black.png` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-mark-black.svg` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-mark.png` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/artcraft-mark.svg` | ArtCraft team (project owner) | https://getartcraft.com/ | ArtCraft trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `assets/app-icon/LICENSE.txt` | Licence history and current layered-rights notice | Earlier upstream terms plus scoped MIXEL revision addendum | Historical MIT OR Apache-2.0; current composite rights under `assets/app-icon/LICENSE-MIXEL-brand.txt` | Earlier notices remain; only original product-symbol layer receives new MIT/Apache grant |
| `assets/app-icon/README.md` | (licence/readme text) | VectorCraft | MIT OR Apache-2.0 |  |
| `assets/app-icon/hicolor/128x128/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 128 px) |
| `assets/app-icon/hicolor/16x16/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 16 px) |
| `assets/app-icon/hicolor/24x24/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 24 px) |
| `assets/app-icon/hicolor/256x256/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 256 px) |
| `assets/app-icon/hicolor/32x32/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 32 px) |
| `assets/app-icon/hicolor/48x48/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 48 px) |
| `assets/app-icon/hicolor/512x512/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 512 px) |
| `assets/app-icon/hicolor/64x64/apps/ch.mixel.vector.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux hicolor icon, 64 px) |
| `assets/app-icon/hicolor/scalable/apps/ch.mixel.vector.svg` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Linux scalable icon) |
| `assets/app-icon/vectorcraft-1024.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (repository full-tile image, 1024 px) |
| `assets/app-icon/vectorcraft-macos-512.png` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (macOS runtime icon, 512 px) |
| `assets/app-icon/vectorcraft-small.svg` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (compact hybrid composition) |
| `assets/app-icon/vectorcraft.icns` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (macOS bundle icon) |
| `assets/app-icon/vectorcraft.ico` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (Windows app icon) |
| `assets/app-icon/vectorcraft.svg` | MIXEL (user-supplied brand); MIXEL contributors (product symbol); Inter Project Authors (name outlines) | User-supplied mixel_core_clean_master.png (SHA-256 d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516); original Bézier control points overlay; outlined Inter SemiBold name; see assets/app-icon/README.md | LicenseRef-MIXEL-Brand (`assets/app-icon/LICENSE-MIXEL-brand.txt`); original symbols MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`); Inter notice `assets/app-icon/OFL-Inter.txt` | MIXEL Vector app icon (canonical hybrid composition) |
| `packaging/macos/dmg/background.svg` | Xus Badia (contributed in #493) | Original artwork for VectorCraft's DMG window; links the app icon `assets/app-icon/vectorcraft-small.svg` | MIT OR Apache-2.0 (licensed by the contributor under the project licence) | DMG background source. Text in Inter and JetBrains Mono (OFL-1.1, `assets/fonts`), not embedded; paper grain generated by an SVG `feTurbulence` filter |
| `packaging/macos/dmg/background.tiff` | Xus Badia (contributed in #493) | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | 1x + 2x DMG background; includes the app icon (see its rows), text rendered from OFL-1.1 fonts and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `packaging/macos/dmg/DS_Store` | Xus Badia (layout, #493) | Written by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | Finder view settings for the DMG (window, icon size and positions, background alias); no machine data |
| `assets/fonts/Inter-Medium.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-Regular.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-SemiBold.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/JetBrainsMono-Regular.ttf` | The JetBrains Mono Project Authors | https://github.com/JetBrains/JetBrainsMono | OFL-1.1 (`assets/fonts/OFL-JetBrainsMono.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Bold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-It.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Regular.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Semibold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSerif4-Regular.ttf` | Frank Grießhammer / Adobe (released as open source) | https://github.com/adobe-fonts/source-serif | OFL-1.1 (`assets/fonts/OFL-SourceSerif4.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/OFL-Inter.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-JetBrainsMono.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSans3.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSerif4.txt` | (licence text) | upstream project | — |  |
| `assets/icons/LICENSE-lucide.txt` | (licence text) | upstream project | — |  |
| `assets/icons/align-center-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-center-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-left-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-up-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/blend.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/brush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chart-column.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/clipboard-paste.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/cloud.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/combine.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/copy.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/dc-actions.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-bottom.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-center.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-hcenter.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-left.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-right.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-top.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-vcenter.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-align.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-artboard.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-key.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-selection.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor-smooth.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-appearance.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arc.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-down.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-extend.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-fill.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 | The Selection and Direct Selection arrows' fill, painted black or white under their outline |
| `assets/icons/dc-arrow-tip.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-up.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboard-options.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboards.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-blend.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-bloat.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-butt.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-round.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-square.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-hide.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-show.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-clear.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-color-guide.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-crystallize.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cube.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-align.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-exact.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-off.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-on.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-direct.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-bottom.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hcenter.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hspace.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-left.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-right.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-top.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vcenter.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vspace.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-behind.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-inside.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-normal.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-ellipse.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fill-none.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-flare.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-folder.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-free-transform.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fx.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gamut.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-freeform.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-linear.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-radial.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-across.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-along.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-within.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gradient.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-graphic-styles.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grid-view.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-group-select.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-bevel.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-miter.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-round.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-knife.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line-cut.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-list-view.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-bucket.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-select.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mask-none.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-measure.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mesh.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mirror-cut.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-fill.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-item.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-stroke.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-options.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-center.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-all.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-center.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-left.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-right.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-left.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-ltr.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-right.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-rtl.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-path-eraser.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pathfinder.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-perspective.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-crop.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-divide.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-exclude.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-intersect.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-merge.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-back.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-front.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-outline.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-trim.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-unite.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-place-symbol.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polar-grid.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polygon.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pucker.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-puppet.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rearrange.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-cut.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-grid.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reference-point.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-remove-brush.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reshape.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reverse.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rotate-view.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rounded-rect.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-evenodd.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-nonzero.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-scallop.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-screen-mode.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-selection.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shape-builder.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shear.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-smooth.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-center.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-inside.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-outside.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swap.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swatch-kinds.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-symbol-sprayer.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-touch-type.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transform-panel.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transparency.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-twirl.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-area.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-path.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-vertical.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width-profile.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-wrinkle.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-large.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-small.svg` | VectorCraft contributors | Original work, drawn for VectorCraft | MIT OR Apache-2.0 |  |
| `assets/icons/ellipsis.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eraser.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/file-plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-horizontal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-vertical-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/folder-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/frame.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/grid-3x3.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/group.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hand.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hexagon.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/history.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/house.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/image.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/info.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lasso.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layers.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layout-grid.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/library.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link-2-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/external-link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) | Help → app page link |
| `assets/icons/git-branch.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → GitHub link |
| `assets/icons/globe.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → website link |
| `assets/icons/message-circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Discord button and Help → Discord link |
| `assets/icons/lock-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lock.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/map.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/maximize-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/menu.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/minus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/monitor.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move-diagonal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paint-bucket.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paintbrush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/palette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool-add.svg` | Lucide Icons and Contributors; badge by VectorCraft contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a plus badge |
| `assets/icons/pen-tool-delete.svg` | Lucide Icons and Contributors; badge by VectorCraft contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a minus badge |
| `assets/icons/pencil.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pilcrow.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pin.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pipette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/printer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/redo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-3d.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-ccw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-cw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ruler.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/save.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scaling.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scissors.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/search.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/settings.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/shapes.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/share-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/slice.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline-pointer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spray-can.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square-dashed.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-exclude.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-intersect.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-subtract.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-unite.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squircle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/star.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sun.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/swatch-book.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/text-cursor-input.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/tornado.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/trash-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/triangle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/type.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/undo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ungroup.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/wand-sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/waves.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/x.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/zoom-in.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `docs/images/dusk-poster.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/dusk-poster.vectorcraft` | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.vectorcraft` | VectorCraft contributors | Original artwork built through the VectorCraft command API | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.svg` | VectorCraft contributors | SVG export of the above | MIT OR Apache-2.0 |  |
| `docs/images/shot-1-neon.png` | VectorCraft contributors | Screenshot of VectorCraft itself (VectorCraft/Lucide UI icons only), editing `examples/neon-drive.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/shot-2-ribbons.png` | VectorCraft contributors | Screenshot of VectorCraft itself, editing `examples/ribbons.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/shot-3-sheet.png` | VectorCraft contributors | Screenshot of VectorCraft itself, editing `examples/feature-sheet.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/shot-4-bezier.png` | VectorCraft contributors | Screenshot of VectorCraft itself, editing `examples/feature-sheet.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/shot-5-perspective.png` | VectorCraft contributors | Screenshot of VectorCraft itself, editing `examples/perspective-city.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/art-neon-drive.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/neon-drive.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/art-ribbons.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/ribbons.vectorcraft` | MIT OR Apache-2.0 |  |
| `docs/images/art-pathfinder.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/feature-sheet.vectorcraft` (artboard 1) | MIT OR Apache-2.0 |  |
| `docs/images/art-mesh.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/feature-sheet.vectorcraft` (artboard 2) | MIT OR Apache-2.0 |  |
| `docs/images/art-repeat.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/feature-sheet.vectorcraft` (artboard 3) | MIT OR Apache-2.0 |  |
| `docs/images/art-envelope.png` | VectorCraft contributors | Rendered by VectorCraft from `examples/feature-sheet.vectorcraft` (artboard 4) | MIT OR Apache-2.0 |  |
| `examples/neon-drive.vectorcraft` | VectorCraft contributors | Original artwork built through the VectorCraft command API | MIT OR Apache-2.0 |  |
| `examples/ribbons.vectorcraft` | VectorCraft contributors | Original artwork built through the VectorCraft command API | MIT OR Apache-2.0 |  |
| `examples/feature-sheet.vectorcraft` | VectorCraft contributors | Original artwork built through the VectorCraft command API | MIT OR Apache-2.0 |  |
| `examples/perspective-city.vectorcraft` | VectorCraft contributors | Original artwork built through the VectorCraft command API | MIT OR Apache-2.0 |  |


## MIXEL font replacement — 2026-10-13

The five Source Sans 3 / Source Serif 4 TTF assets listed above are retired and no longer present or embedded in MIXEL Vector. Their original credit rows and OFL licence texts are retained as historical upstream notices. The owner-authorized replacements below keep the existing Latin font footprint at five faces. The UI uses the already bundled Inter family; document text uses Noto Sans and Noto Serif. The Noto binaries are unmodified official files pinned to source commit `4e15001b9fb484cc871d7b2ad930cd2226edf260`; their sidecars record SHA-256 checksums.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/fonts/NotoSans-Regular.ttf` | The Noto Project Authors | https://raw.githubusercontent.com/notofonts/notofonts.github.io/4e15001b9fb484cc871d7b2ad930cd2226edf260/fonts/NotoSans/unhinted/ttf/NotoSans-Regular.ttf | OFL-1.1 (`assets/fonts/OFL-Noto.txt`) | Unmodified official static face; SHA-256 `f3961a9cde016d41a4879aecda1474d3a36d6bf54fa0e4643de029cc2248b0e8` |
| `assets/fonts/NotoSans-Regular.ttf.attribution` | MIXEL | Original provenance record for the font above | MIT OR Apache-2.0 | Source commit and SHA-256 pin |
| `assets/fonts/NotoSans-SemiBold.ttf` | The Noto Project Authors | https://raw.githubusercontent.com/notofonts/notofonts.github.io/4e15001b9fb484cc871d7b2ad930cd2226edf260/fonts/NotoSans/unhinted/ttf/NotoSans-SemiBold.ttf | OFL-1.1 (`assets/fonts/OFL-Noto.txt`) | Unmodified official static face; SHA-256 `a7af8d4a11e8e49fea9a19786a6469210b512cd4061413e8036bdd2425722795` |
| `assets/fonts/NotoSans-SemiBold.ttf.attribution` | MIXEL | Original provenance record for the font above | MIT OR Apache-2.0 | Source commit and SHA-256 pin |
| `assets/fonts/NotoSans-Bold.ttf` | The Noto Project Authors | https://raw.githubusercontent.com/notofonts/notofonts.github.io/4e15001b9fb484cc871d7b2ad930cd2226edf260/fonts/NotoSans/unhinted/ttf/NotoSans-Bold.ttf | OFL-1.1 (`assets/fonts/OFL-Noto.txt`) | Unmodified official static face; SHA-256 `87cb2d84472a7d66da659ee47b6cdb9552326e8c128245231f191b6ac72529d9` |
| `assets/fonts/NotoSans-Bold.ttf.attribution` | MIXEL | Original provenance record for the font above | MIT OR Apache-2.0 | Source commit and SHA-256 pin |
| `assets/fonts/NotoSans-Italic.ttf` | The Noto Project Authors | https://raw.githubusercontent.com/notofonts/notofonts.github.io/4e15001b9fb484cc871d7b2ad930cd2226edf260/fonts/NotoSans/unhinted/ttf/NotoSans-Italic.ttf | OFL-1.1 (`assets/fonts/OFL-Noto.txt`) | Unmodified official static face; SHA-256 `678288f868807d4d64a6f3b51466871d117d915780381ce9d0ed4b3bcbd06d37` |
| `assets/fonts/NotoSans-Italic.ttf.attribution` | MIXEL | Original provenance record for the font above | MIT OR Apache-2.0 | Source commit and SHA-256 pin |
| `assets/fonts/NotoSerif-Regular.ttf` | The Noto Project Authors | https://raw.githubusercontent.com/notofonts/notofonts.github.io/4e15001b9fb484cc871d7b2ad930cd2226edf260/fonts/NotoSerif/unhinted/ttf/NotoSerif-Regular.ttf | OFL-1.1 (`assets/fonts/OFL-Noto.txt`) | Unmodified official static face; SHA-256 `a15cfbbc1539d707115111d672d590a3d70d4f74b4c0a315956da20ae19a14e1` |
| `assets/fonts/NotoSerif-Regular.ttf.attribution` | MIXEL | Original provenance record for the font above | MIT OR Apache-2.0 | Source commit and SHA-256 pin |
| `assets/fonts/OFL-Noto.txt` | The Noto Project Authors | https://github.com/notofonts/latin-greek-cyrillic/blob/main/OFL.txt | OFL-1.1 | Full licence text for Noto Sans and Noto Serif |


## MIXEL runtime trademark policy — 2026-10-13

The `docs/brand/` ArtCraft logo and mark rows remain as historical upstream credits. Those files are not used as MIXEL runtime marks or README artwork. The runtime brand mark is the original MIT/Apache-2.0 engraved-dragon app icon already attributed above, and the README retains plain-text ArtCraft credit.

## MIXEL UI symbol fallback — 2026-10-13

The existing unmodified JetBrains Mono Regular face also supplies UI symbols missing from Inter, including U+25BE (BLACK DOWN-POINTING SMALL TRIANGLE). It follows the current proportional UI fonts and remains first for the mono family. Its bytes match the official immutable source above at commit `19371302b95d218af43299bce79ddbddd0bc364d`, SHA-256 `e6fd0d7e91550b3ed2b735d4312474362c4716edc4fc0577a0f61ed782d5aed1`; the existing complete OFL text matches the same commit, SHA-256 `a76abf002c49097d146e86740a3105a5d00450b1592e820a1109a8c5680cd697`. No new font binary or modified glyph is added.

| Asset | Author | Source | Licence | Notes |
| --- | --- | --- | --- | --- |
| `assets/fonts/JetBrainsMono-Regular.ttf.attribution` | MIXEL | Original provenance record; [pinned official font](https://raw.githubusercontent.com/JetBrains/JetBrainsMono/19371302b95d218af43299bce79ddbddd0bc364d/fonts/ttf/JetBrainsMono-Regular.ttf) | MIT OR Apache-2.0 | Records the existing OFL face and SHA-256 pin; the original font credit row above remains unchanged |


## MIXEL supplied-brand app-icon revision — 2026-10-15

The project owner selected the exact supplied MIXEL brand master for these application icons. This narrowly scoped first-party brand use is additional to the earlier ArtCraft-brand exception; it does not permit unrelated non-open assets. The supplied base and branded composites have reserved rights under `assets/app-icon/LICENSE-MIXEL-brand.txt`. Only the independent new product-symbol layers have the separate MIT OR Apache-2.0 contributor grant. Inter SemiBold name outlines retain the full notice in `assets/app-icon/OFL-Inter.txt`; no new font binary is included. Earlier notices remain applicable to unchanged assets and earlier derivatives.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/LICENSE-MIXEL-brand.txt` | MIXEL project owner (authorization record; existing rights retained by respective holders) | User-supplied master and requested application composition | Reserved-brand terms | Current MIXEL composite icon terms; not an open-source grant |
| `assets/app-icon/OFL-Inter.txt` | The Inter Project Authors | Exact existing `assets/fonts/OFL-Inter.txt` | OFL-1.1 | Full notice for outlined Inter SemiBold product names |

<details>
<summary>Historical metadata for the replaced upstream app-icon revision</summary>

```text
| `assets/app-icon/LICENSE.txt` | (licence/readme text) | VectorCraft | MIT OR Apache-2.0 |  |
| `assets/app-icon/hicolor/128x128/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/16x16/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/24x24/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/256x256/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/32x32/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/48x48/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/512x512/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/64x64/apps/ch.mixel.vector.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/scalable/apps/ch.mixel.vector.svg` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft-1024.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft-macos-512.png` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft-small.svg` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft.icns` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft.ico` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/vectorcraft.svg` | Project owner (ArtCraft team) | Original artwork drawn in ArtCraft for VectorCraft, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
```

</details>
