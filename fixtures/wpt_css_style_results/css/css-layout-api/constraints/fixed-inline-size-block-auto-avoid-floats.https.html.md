# css/css-layout-api/constraints/fixed-inline-size-block-auto-avoid-floats.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-block-auto-avoid-floats.https.html"
}
```

## style[0]

```css

body {
  width: 200px;
}

.float {
  float: right;
  width: 100px;
  height: 20px;
}

.test {
  background: red;
}

@supports (display: layout(test)) {
  .test {
    background: green;
    display: layout(test);
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
