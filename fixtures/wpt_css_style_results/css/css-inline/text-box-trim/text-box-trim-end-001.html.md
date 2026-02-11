# css/css-inline/text-box-trim/text-box-trim-end-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-end-001.html"
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
  text-box-trim: trim-end;
}
.vrl { writing-mode: vertical-rl; }
.vlr { writing-mode: vertical-lr; }
.auto       .target { text-box-edge: auto; }
.text       .target { text-box-edge: text; }
.alphabetic .target { text-box-edge: text alphabetic; }
.cap        .target { text-box-edge: cap text; }
.ex         .target { text-box-edge: ex text; }
```

```json
{
  "errors": 7,
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
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
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
