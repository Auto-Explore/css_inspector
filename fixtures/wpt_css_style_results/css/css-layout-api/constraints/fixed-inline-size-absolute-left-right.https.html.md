# css/css-layout-api/constraints/fixed-inline-size-absolute-left-right.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-absolute-left-right.https.html"
}
```

## style[0]

```css

body {
  position: relative;
  width: 120px;
}

.test {
  background: red;
  position: absolute;
  left: 0px;
  right: 20px;
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
