# css/css-inline/text-box-trim/text-box-trim-start-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-start-001.html"
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
  text-box-trim: trim-start;
}
.vrl { writing-mode: vertical-rl; }
.vlr { writing-mode: vertical-lr; }
.auto .target { text-box-edge: auto; }
.text .target { text-box-edge: text; }
.cap  .target { text-box-edge: cap text; }
.ex   .target { text-box-edge: ex text; }
.vlr.alphabetic
      .target { text-box-edge: text alphabetic; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
