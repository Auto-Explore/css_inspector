# css/css-inline/text-box-trim/text-box-trim-block-in-inline-start-003.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-block-in-inline-start-003.html"
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
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-trim”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
