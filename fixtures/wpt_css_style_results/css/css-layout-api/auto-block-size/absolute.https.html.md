# css/css-layout-api/auto-block-size/absolute.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/auto-block-size/absolute.https.html"
}
```

## style[0]

```css

.test {
  position: absolute;
  background: red;
}

@supports (display: layout(block-size-100)) {
  .test {
    display: layout(block-size-100);
    background: green;
  }
}

.container {
  position: relative;
  margin: 20px 0;
  width: 200px;
  height: 200px;
  border: solid 2px;
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
