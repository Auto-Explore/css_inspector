# css/css-inline/text-box-trim/text-box-trim-accumulation-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-accumulation-004-ref.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

body {
  font: 100px/1 MetricsTestFont;
}
.spacer {
  background: lightgray;
  block-size: 100px;
}
.target-first {
  margin-block: -60px 0px;
}
.target-second {
  margin-block: 0px -20px;
}
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
      "message": "Invalid value for property “margin-block”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “margin-block”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
