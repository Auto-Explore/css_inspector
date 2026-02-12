# css/css-inline/text-box-trim/text-box-trim-end-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-end-001-ref.html"
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
}
.offset {
  position: relative;
}
.vrl { writing-mode: vertical-rl; }
.vlr { writing-mode: vertical-lr; }
.auto .target, .text .target { line-height: 150px; }
.auto .offset, .text .offset { inset-block-start: 25px; }
.alphabetic .target { line-height: 130px; }
.alphabetic .offset { inset-block-start: 35px;}
.cap .target { line-height: 130px; }
.cap .offset { inset-block-start: -35px; }
.ex .target { line-height: 90px; }
.ex .offset { inset-block-start: -55px; }
.vlr.cap .offset { inset-block-start: 35px; }
.vlr.ex .offset { inset-block-start: 55px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
