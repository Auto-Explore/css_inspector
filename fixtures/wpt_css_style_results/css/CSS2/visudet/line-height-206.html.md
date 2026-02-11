# css/CSS2/visudet/line-height-206.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/visudet/line-height-206.html"
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
  position: absolute;
  line-height: normal;
  font-size: 100px;
  color: transparent;
  border: solid black 1px;
}

.h { font-family: high-a-only; }
.dh { font-family: deep-b-only, high-a-only; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
