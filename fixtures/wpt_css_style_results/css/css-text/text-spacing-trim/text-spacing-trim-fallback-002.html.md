# css/css-text/text-spacing-trim/text-spacing-trim-fallback-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-spacing-trim/text-spacing-trim-fallback-002.html"
}
```

## style[0]

```css

@font-face {
  font-family: halt-font;
  font-display: block;
  src: url('/fonts/noto/cjk/NotoSansCJKjp-Regular-subset-halt.otf');
}
@font-face {
  font-family: chws-font;
  font-display: block;
  src: url('/fonts/noto/cjk/NotoSansCJKjp-Regular-subset-chws.otf');
}
#container {
  font-family: Arial, halt-font;
  font-size: 20px;
}
.chws #container {
  font-family: Arial, chws-font;
}
:root.vrl {
  writing-mode: vertical-rl;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
