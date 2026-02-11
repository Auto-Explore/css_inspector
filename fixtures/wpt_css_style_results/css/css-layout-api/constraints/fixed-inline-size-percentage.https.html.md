# css/css-layout-api/constraints/fixed-inline-size-percentage.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-percentage.https.html"
}
```

## style[0]

```css

body {
  width: 200px;
}

.test {
  background: red;
  width: 50%;
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
