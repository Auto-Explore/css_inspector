# css/css-inline/text-box-trim/text-box-trim-float-clear-br-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-float-clear-br-003-ref.html"
}
```

## style[0]

```css

@import "support/MetricsTestFont.css";

.spacer {
  margin-top: -10px;
  background: lightgray;
  block-size: 100px;
}
.target {
  font-family: MetricsTestFont;
  font-size: 100px;
  line-height: 1;
}
.float {
  float: left;
  width: 100px;
  height: 90px;
  background: yellow;
}
.clear { clear: both; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
