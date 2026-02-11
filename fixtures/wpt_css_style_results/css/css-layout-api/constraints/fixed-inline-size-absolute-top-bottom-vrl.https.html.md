# css/css-layout-api/constraints/fixed-inline-size-absolute-top-bottom-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-absolute-top-bottom-vrl.https.html"
}
```

## style[0]

```css

body {
  position: relative;
  height: 120px;
}

.test {
  background: red;
  position: absolute;
  left: 0px;
  top: 0px;
  bottom: 20px;
  writing-mode: vertical-rl;
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
