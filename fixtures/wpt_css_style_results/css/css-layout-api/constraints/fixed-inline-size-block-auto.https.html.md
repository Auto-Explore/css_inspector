# css/css-layout-api/constraints/fixed-inline-size-block-auto.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-block-auto.https.html"
}
```

## style[0]

```css

body {
  width: 120px;
}

.test {
  margin-right: 20px;
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
