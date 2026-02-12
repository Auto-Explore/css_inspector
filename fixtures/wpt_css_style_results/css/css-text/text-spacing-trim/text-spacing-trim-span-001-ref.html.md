# css/css-text/text-spacing-trim/text-spacing-trim-span-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-spacing-trim/text-spacing-trim-span-001-ref.html"
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
  inline-size: 4em;
  text-spacing-trim: space-all;
}
.vrl {
  writing-mode: vertical-rl;
}
halt {
  font-feature-settings: 'halt' 1, 'vhal' 1;
}
.trim-start {
  ps, ls { font-feature-settings: 'halt' 1, 'vhal' 1; }
}
.space-first {
  ls { font-feature-settings: 'halt' 1, 'vhal' 1; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
