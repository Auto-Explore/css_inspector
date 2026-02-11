# css/CSS2/visudet/reference/content-height-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/visudet/reference/content-height-003-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: 'high-a-only';
  font-style: normal;
  font-weight: 400;
  src:  url(/fonts/Revalia.woff) format('woff');
  unicode-range: U+0061, U+0020;
}
@font-face {
  font-family: 'deep-b-only';
  font-style: normal;
  font-weight: 400;
  src: url(/fonts/AD.woff) format('woff');
  unicode-range: U+0062, U+0020;
}

div {
  font-size: 50px;
  display: inline-block;
  color: transparent;
  font-family: high-a-only, deep-b-only;
}

span { background: blue; }

div { line-height: 200px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
