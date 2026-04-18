# Design System Specification: Editorial Noir

## 1. Overview & Creative North Star

The Creative North Star for this design system is **"The Digital Monograph."**

We are moving away from the "app-like" interface and toward a high-end editorial experience that feels curated, intellectual, and authoritative. This system leverages the stark, unapologetic contrast of pure black (`#000000`) and pure white (`#ffffff`) to create a canvas where content is the singular focus.

By utilizing negative letter-spacing in headings and classical serif body text, we create a "New York Times meets IDE" aesthetic. The design breaks the traditional grid through intentional asymmetry, extreme vertical breathing room, and a "No-Line" philosophy that relies on tonal depth rather than structural scaffolding.

---

## 2. Colors & Surface Architecture

### The Palette

The color strategy is binary, punctuated by a singular intellectual blue and a disruptive crimson interaction state.

| Role                   | Light Mode | Dark Mode | Token                    |
| :--------------------- | :--------- | :-------- | :----------------------- |
| **Background**         | `#ffffff`  | `#000000` | `surface`                |
| **Surface (Low)**      | `#F7F7F7`  | `#0e0e0e` | `surface-container-low`  |
| **Surface (High)**     | `#ebebeb`  | `#111111` | `surface-container-high` |
| **Primary Accent**     | `#2b2ed4`  | `#2b2ed4` | `primary`                |
| **Interaction**        | `#cf2d56`  | `#cf2d56` | `secondary`              |
| **Tertiary/Highlight** | `#ffffff`  | `#ffffff` | `tertiary`               |
| **Text (Primary)**     | `#000000`  | `#ffffff` | `on-surface`             |

### The "No-Line" Rule

Prohibit 1px solid borders for sectioning. Structural boundaries must be defined solely through background color shifts. A sidebar should not be "bordered" off; it should simply exist on `surface-container-low` against a `surface` background.

### Glass & Gradient Implementation

To prevent the UI from feeling flat or "default," use Glassmorphism for floating utilities (e.g., command palettes, hover menus).

- **Formula:** `surface-variant` + `80% Alpha` + `24px Backdrop Blur`.
- **CTAs:** Use a subtle linear gradient for primary buttons, transitioning from `primary` to `primary-container` at a 145° angle. This adds "soul" and weight to the interaction point.

---

## 3. Typography

Typography is the heartbeat of this system. It transitions from the brutalist density of the headings to the literary elegance of the body.

- **Display & Headlines (Space Grotesk):** Used for primary messaging. Set with `letter-spacing: -0.04em`. This "tight" setting creates a custom, high-fashion look that feels engineered and intentional.
- **Body (Newsreader):** Used for all long-form reading. This serif choice signals high-end editorial quality. It provides a humanistic counter-balance to the coldness of the black-and-white palette.
- **Labels & UI (Inter):** Used for functional text and metadata to maintain maximum clarity and precision.
- **Interaction States:** Upon hover, interactive text elements shift from `on-surface` to `secondary` (Crimson). This is a visceral, high-contrast change that provides immediate feedback without needing an underline.

---

## 4. Elevation & Depth

### The Layering Principle

Depth is achieved by "stacking" surface-container tiers.

- **Level 0:** `surface` (The Page)
- **Level 1:** `surface-container-low` (Content Groupings)
- **Level 2:** `surface-container-high` (Active Elements/Cards)

### Ambient Shadows

For floating elements (Modals, Popovers), use "Atmospheric Shadows."

- **Light Mode:** `0px 24px 48px rgba(0, 0, 0, 0.06)`
- **Dark Mode:** `0px 24px 48px rgba(0, 0, 0, 0.4)`
  Shadows must be ultra-diffused. If a shadow looks like a "line," it is too sharp. It should feel like a soft glow of darkness or light.

### The "Ghost Border" Fallback

If accessibility requirements demand a border, use the `outline-variant` token at **10% opacity**. It should be felt, not seen.

---

## 5. Components

### Buttons

- **Primary:** Background `primary`, Text `on-primary`, Radius `8px` (Moderate roundedness). Use a 50% width "shimmer" gradient on hover.
- **Secondary:** Background `transparent`, Border `Ghost Border`, Text `on-surface`.
- **Tertiary:** No background. Text `on-surface`. On hover, text color shifts to `secondary` (Crimson).

### Chips & Tags

- **Style:** Pill-shaped curves (Moderate roundedness).
- **Visuals:** Use `surface-container-highest` for the background with `Inter` labels. This creates a technical "tagging" feel.

### Input Fields

- **Style:** Underline-only or subtle `surface-container-low` fills.
- **Active State:** The border transitions to `primary` (Deep Blue) with a 2px thickness. Labels use `inter` to ensure maximum legibility at small sizes.

### Cards & Lists

- **The Divider Ban:** Explicitly forbid horizontal divider lines. Use Normal spacing (`spacing: 2`) as the base, relying on vertical white space to separate list items. If separation is visually impossible without a container, use a subtle background shift to `surface-container-lowest`.

---

## 6. Do's and Don'ts

### Do

- **Do** embrace the moderate "Normal" spacing. If a layout feels cluttered, increase the breathing room.
- **Do** use `Newsreader` for anything meant to be read as a sentence.
- **Do** use negative letter-spacing on all headings above 24px.
- **Do** let the background "bleed" through floating elements using backdrop-blur.

### Don't

- **Don't** use grey `#808080` for text. Use the provided surface tokens or alpha-transparency on white/black to maintain the pure chromatic scale.
- **Don't** use 100% opaque borders. They clutter the visual field and break the editorial flow.
- **Don't** use standard "Blue" for links. Use the `primary` blue for actions and `secondary` crimson for hovers.
- **Don't** use shadows on nested cards. Use tonal layering instead.
