# css/css-text/text-spacing-trim/text-spacing-trim-dot-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-spacing-trim/text-spacing-trim-dot-001-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: halt-font;
  src: url('/fonts/noto/cjk/NotoSansCJKjp-Regular-subset-halt.otf');
}
#container {
  font-family: halt-font;
  font-size: 20px;
  position: relative;
  text-spacing-trim: space-all;
}
#container > div {
  position: absolute;
}
:root:not(.vrl) .col1 {
  left: 6em;
}
:root:not(.vrl) .col2 {
  left: 12em;
}
:root.vrl {
  writing-mode: vertical-rl;
}
:root.vrl .col1 {
  top: 6em;
}
:root.vrl div.col2 {
  top: 12em;
}
halt {
  font-feature-settings: 'halt' 1, 'vhal' 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
