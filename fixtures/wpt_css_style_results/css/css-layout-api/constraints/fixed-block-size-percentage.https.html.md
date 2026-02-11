# css/css-layout-api/constraints/fixed-block-size-percentage.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-percentage.https.html"
}
```

## style[0]

```css

body {
  height: 200px;
}

.test {
  background: red;
  --expected-block-size: 100;
  width: 100px;
  height: 50%;
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
