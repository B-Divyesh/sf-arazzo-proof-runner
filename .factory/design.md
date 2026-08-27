# Visual thesis: poured proof

## Direction and rationale

Arazzo Proof Runner uses **brutalist concrete and moss**: a dense, daylight utility surface that feels like evidence pinned to a poured-concrete test bench. Concrete represents stable, repeatable artifacts; moss marks the living paths and outputs that connect workflow steps. The design avoids glossy API-client conventions and generic gradient heroes. It should read as an instrument, not a campaign page.

The interface has two treatments. The default is chalk concrete in daylight; `prefers-color-scheme: dark` becomes wet charcoal concrete with lichen highlights. Both preserve the same status semantics.

## Tokens

| Role | Light | Dark | Use |
| --- | --- | --- | --- |
| Background | `#efede4` | `#171a17` | Poured-concrete field |
| Surface | `#d8d3c5` | `#252a25` | Slabs and code wells |
| Raised surface | `#f8f6ef` | `#303630` | Evidence strips |
| Text | `#171a16` | `#f1efe6` | Primary copy |
| Muted text | `#555c51` | `#b8c0b3` | Supporting copy |
| Moss | `#365b32` | `#a9c978` | Links and positive state |
| Moss contrast | `#ffffff` | `#11150f` | Text on moss |
| Lichen | `#bbcf72` | `#c9dd82` | Highlight and focus |
| Rust | `#9b3528` | `#ff9a86` | Failure and warning |
| Hairline | `#7a7f74` | `#71796f` | Structural rules |

Status is never color-only: every state has a word and a geometric mark (`✓`, `×`, or `—`). All body combinations target WCAG AA (4.5:1); structural outlines target 3:1.

## Type and rhythm

- **Utility sans:** `Arial`, `Helvetica Neue`, system sans. Blunt headings, 700 weight.
- **Evidence mono:** `ui-monospace`, `SFMono-Regular`, `Consolas`, monospace. Commands, identifiers, results, and tabular figures.
- No remote or bundled font payload is required. Body text is 16–18px with 1.55 leading and a 68-character reading measure.
- Spacing follows an 8px base with 4px for tight evidence rows: `4, 8, 16, 24, 32, 48, 72, 96`.
- Corners are nearly square (`0–4px`). Heavy 2px rules and offset shadows make layers legible without ornamental cards.

## Composition and interaction grammar

- A vertical workflow seam connects numbered steps; changing state appears as a moss proof stamp crossing that seam.
- Primary actions are moss blocks with a 2px ink edge. Secondary actions are underlined text or concrete blocks.
- Hover lifts a slab by 2px and deepens its offset shadow. Pressing returns it to the plane. Keyboard focus is a 3px lichen/ink double ring.
- At 390px, navigation becomes a compact two-row ledger, the proof table becomes stacked evidence strips, and nonessential specimen annotations disappear. No action target is under 44px.

## Motion policy

Only stateful motion is used: slabs settle from 4px above over 180ms; result stamps enter from the workflow seam over 220ms. Transforms and opacity only. Nothing loops. Under `prefers-reduced-motion: reduce`, transitions and transforms are removed and state changes are immediate.

## Original asset plan and provenance

- `site/public/proof-strata.webp`: generated hero illustration, a top-down brutalist concrete testing bench with three connected evidence slabs and organic moss tracing the workflow seam. No text, logos, UI screenshots, people, or trademarks. Generated specifically for this product with the factory image deployment via `/opt/fleet/lib/gen-image.sh`, then converted to WebP at ≤300 KB. Prompt and deployment metadata are stored beside the source during generation; only the optimized WebP ships.
- The small step, arrow, check, and failure marks are hand-authored CSS/geometric glyphs, not third-party icons.

Final generation record (2026-08-27): factory CLI mode, `/opt/fleet/lib/gen-image.sh`, `factory-image` deployment, 1536×1024, high quality. Prompt: “Use case: stylized-concept. Asset type: landing page hero illustration. Primary request: top-down editorial still life of three rough cast-concrete slabs connected as a precise API workflow, with thin living moss following the seams between slabs and tiny stamped paper evidence tabs without readable text. Scene/backdrop: tactile aggregate workshop surface with chipped concrete edges. Style/medium: brutalist architectural photography, highly tactile but restrained. Composition/framing: wide landscape, three clear stages moving diagonally, quiet negative space around the objects. Lighting/mood: overcast workshop daylight, rigorous and calm. Color palette: chalk concrete, charcoal, deep moss, restrained pale lichen. Constraints: original image; no people; no screens; no readable text; no logos; no trademarks; no watermark; no gradients.” The PNG was inspected, then encoded to WebP at quality 72 (208 KB). The final project-owned asset is `site/public/proof-strata.webp`; no stock or third-party visual assets are used.
