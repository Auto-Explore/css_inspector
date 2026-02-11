# css/css-layout-api/constraints/fixed-inline-size-flex-grow-column-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-flex-grow-column-vrl.https.html"
}
```

## style[0]

```css

body {
  display: flex;
  flex-flow: column;
  width: 100px;
  height: 100px;
}

.test {
  background: red;
  flex-grow: 1;
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
