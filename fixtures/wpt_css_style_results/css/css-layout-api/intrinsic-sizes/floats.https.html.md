# css/css-layout-api/intrinsic-sizes/floats.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/floats.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  border: solid 2px;
}

@supports (display: layout(test-layout)) {
  .test {
    display: layout(test-layout);
    background: green;
  }
}

.container-1 {
  height: 100px;
  width: 300px;
}

.container-2 {
  height: 50px;
  width: 50px;
}

.container-3 {
  height: 80px;
  width: 80px;
}

.left {
  float: left;
  writing-mode: horizontal-tb;
}

.right {
  float: right;
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
