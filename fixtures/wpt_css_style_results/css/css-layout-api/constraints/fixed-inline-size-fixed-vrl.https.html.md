# css/css-layout-api/constraints/fixed-inline-size-fixed-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-fixed-vrl.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  height: 100px;
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
