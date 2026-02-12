# css/css-flexbox/flex-container-min-content-002.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-container-min-content-002.tentative.html"
}
```

## style[0]

```css

.flex {
  display: inline-flex;
  vertical-align: top;
  border: 5px solid magenta;
  width: min-content;
  height: min-content;
}
.flex.min {
  width: 0;
  height: 0;
  min-width: min-content;
  min-height: min-content;
}
.flex.max {
  width: 200px;
  height: 200px;
  max-width: min-content;
  max-height: min-content;
}
.flex > div {
  font: 25px/1 Ahem;
  border: 5px solid cyan;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
