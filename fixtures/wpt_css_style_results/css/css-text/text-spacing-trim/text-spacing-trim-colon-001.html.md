# css/css-text/text-spacing-trim/text-spacing-trim-colon-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-spacing-trim/text-spacing-trim-colon-001.html"
}
```

## style[0]

```css

@font-face {
  font-family: halt-font;
  src: url('/fonts/noto/cjk/NotoSansCJKjp-Regular-subset-halt.otf');
}
@font-face {
  font-family: chws-font;
  src: url('/fonts/noto/cjk/NotoSansCJKjp-Regular-subset-chws.otf');
}
#container {
  font-family: halt-font;
  font-size: 20px;
  position: relative;
}
.chws #container {
  font-family: chws-font;
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
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
