# css/css-inline/text-box-trim/text-box-trim-accumulation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-accumulation-001.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

.spacer {
  block-size: 100px;
  background: lightgray;
}

.inner {
  font: 100px/2 MetricsTestFont;
}
.outer {
  text-box-trim: trim-start;
  text-box-edge: auto;
}

.middle {
  text-box-trim: trim-end;
  text-box-edge: text alphabetic;
}

.inner:first-child {
  text-box-edge: ex alphabetic;
}

.inner:last-child {
  text-box-edge: text;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
