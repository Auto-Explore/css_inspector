# css/css-layout-api/constraints/fixed-inline-size-fixed.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-fixed.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
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
