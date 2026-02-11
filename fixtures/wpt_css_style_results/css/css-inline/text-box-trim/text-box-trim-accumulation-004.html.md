# css/css-inline/text-box-trim/text-box-trim-accumulation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-accumulation-004.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

.spacer {
  block-size: 100px;
  background: lightgray;
}

.outer {
  text-box-trim: trim-end;
  text-box-edge: ex alphabetic;
}

.middle {
  text-box-trim: trim-start;
  text-box-edge: ex alphabetic;
}

.inner {
  font: 100px/1 MetricsTestFont;
}
```

```json
{
  "errors": 5,
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
