# css/css-layout-api/auto-block-size/inflow.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/auto-block-size/inflow.https.html"
}
```

## style[0]

```css

.test {
  margin: 20px 0;
  background: red;
}

@supports (display: layout(block-size-100)) {
  .test {
    display: layout(block-size-100);
    background: green;
  }
}

.width-100 {
  width: 100px;
  writing-mode: horizontal-tb;
}

.height-100 {
  height: 100px;
  writing-mode: vertical-rl;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
