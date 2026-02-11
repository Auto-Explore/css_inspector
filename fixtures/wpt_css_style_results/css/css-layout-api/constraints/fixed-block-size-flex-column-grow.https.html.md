# css/css-layout-api/constraints/fixed-block-size-flex-column-grow.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-column-grow.https.html"
}
```

## style[0]

```css

body {
  display: flex;
  flex-direction: column;
  width: 100px;
  height: 60px;
}

.test {
  background: red;
  --expected-block-size: 50;
  margin-bottom: 10px;
  flex-grow: 1;
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
