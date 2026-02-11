# css/css-layout-api/constraints/fixed-block-size-flex-column-basis.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-column-basis.https.html"
}
```

## style[0]

```css

body {
  display: flex;
  flex-direction: column;
  width: 100px;
}

.test {
  background: red;
  --expected-block-size: 60;
  flex-basis: 60px;
}

.child {
  background: green;
}

@supports (display: layout(test)) {
  .test {
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
