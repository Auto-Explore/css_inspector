# css/css-layout-api/auto-block-size/negative.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/auto-block-size/negative.https.html"
}
```

## style[0]

```css


.test {
  background: red;
  border: solid 2px;
  margin: 20px 0;
}

.width-100 {
  width: 100px;
  writing-mode: horizontal-tb;
}

.height-100 {
  height: 100px;
  writing-mode: vertical-rl;
}

@supports (display: layout(block-size-negative)) {
  .test {
    display: layout(block-size-negative);
    background: green;
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
