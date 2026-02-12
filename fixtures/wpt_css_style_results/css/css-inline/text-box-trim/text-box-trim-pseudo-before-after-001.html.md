# css/css-inline/text-box-trim/text-box-trim-pseudo-before-after-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-pseudo-before-after-001.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 50px;
}
.target {
  font: 50px/2 Ahem;
  text-box-trim: trim-start;
  text-box-edge: text;
}
.target::before {
  content: "X";
  color: blue;
}
.target::after {
  content: "X";
  color: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
