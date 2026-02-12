# css/css-inline/text-box-trim/text-box-trim-float-clear-br-002.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-float-clear-br-002.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

.spacer {
  background: lightgray;
  block-size: 100px;
}
.target, .float {
  font: 100px/1 MetricsTestFont;
}
.target {
  text-box-trim: trim-end;
  text-box-edge: text alphabetic;
}
.float {
  float: left;
  color: yellow;
}
.clear { clear: both; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
