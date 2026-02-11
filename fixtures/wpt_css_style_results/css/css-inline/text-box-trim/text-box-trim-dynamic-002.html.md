# css/css-inline/text-box-trim/text-box-trim-dynamic-002.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-dynamic-002.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

.spacer {
  background: lightgray;
  block-size: 100px;
}
.target {
  font: 100px/2 MetricsTestFont;
  text-box-trim: trim-both;
}
.trim { text-box-edge: ex alphabetic; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
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
  "warnings": 1
}
```
