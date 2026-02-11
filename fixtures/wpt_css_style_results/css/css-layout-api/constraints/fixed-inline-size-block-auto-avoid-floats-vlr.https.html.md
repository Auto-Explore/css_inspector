# css/css-layout-api/constraints/fixed-inline-size-block-auto-avoid-floats-vlr.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-block-auto-avoid-floats-vlr.https.html"
}
```

## style[0]

```css

body {
  height: 200px;
  writing-mode: vertical-lr;
}

.float {
  float: right;
  width: 20px;
  height: 100px;
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
