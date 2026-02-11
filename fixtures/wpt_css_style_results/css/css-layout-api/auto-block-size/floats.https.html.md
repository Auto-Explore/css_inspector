# css/css-layout-api/auto-block-size/floats.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/auto-block-size/floats.https.html"
}
```

## style[0]

```css

.test {
  background: red;
}

@supports (display: layout(block-size-100)) {
  .test {
    display: layout(block-size-100);
    background: green;
  }
}

.container {
  width: 150px;
  border: solid 2px;
}

.left {
  float: left;
  width: 100px;
  writing-mode: horizontal-tb;
}

.right {
  float: right;
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
