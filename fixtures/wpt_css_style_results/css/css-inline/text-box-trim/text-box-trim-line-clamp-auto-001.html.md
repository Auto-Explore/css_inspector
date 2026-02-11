# css/css-inline/text-box-trim/text-box-trim-line-clamp-auto-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-line-clamp-auto-001.html"
}
```

## style[0]

```css

.spacer {
  background: lightgray;
  block-size: 100px;
}
.target {
  font: 50px/2 Ahem;
  text-box-trim: trim-end;
  text-box-edge: text;

  line-clamp: auto;
  max-height: 285px;
}
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
