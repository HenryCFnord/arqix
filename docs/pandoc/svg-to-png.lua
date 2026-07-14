-- PDF-only Pandoc filter: rewrite the committed C4 architecture-view embeds
-- from their SVG to their PNG variant (ADR-0016, REQ-04-01-18-01).
--
-- The committed SVGs use a generic `font-family="sans-serif"` that rsvg-convert
-- cannot resolve inside the render container, so their labels come out as tofu.
-- The PNG variant has its text baked to pixels at Kroki render time (the
-- digest-pinned PlantUML font stack), so the PDF never resolves a font for them.
-- Only the `model/generated/*` C4 views are rewritten; the website `zensical
-- build` keeps the scalable SVG because this filter runs on the PDF path only
-- (it is referenced from docs/pandoc/defaults.yaml, not the site toolchain).
function Image(img)
  img.src = img.src:gsub("(model/generated/[%w%-_]+)%.svg$", "%1.png")
  return img
end
