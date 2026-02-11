# css/css-scroll-anchoring/abspos-in-multicol-002.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/abspos-in-multicol-002.html"
}
```

## style[0]

```css

html {
  column-count: 1; /* Fragmentainer */
}

body {
  position: relative; /* Containing block */
  height: 4000px;
}

main {
  height: 0px; /* Fully clipped */
}

div {
  position: absolute; /* Abspos */
  font-size: 100px;
  width: 200px;
  height: 100%;
  line-height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
