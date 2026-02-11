# css/css-inline/text-box-trim/text-box-trim-pseudo-before-after-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-pseudo-before-after-002-ref.html"
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
  height: 75px;
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
