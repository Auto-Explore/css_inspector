# css/css-layout-api/constraints/fixed-block-size-flex-stretch-max.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-stretch-max.https.html"
}
```

## style[0]

```css

body {
  display: flex;
  height: 50px;
}

.test {
  background: red;
  --expected-block-size: 30; /* flex-item should respect the max constraint */
  max-height: 30px;
  width: 100px;
  margin-bottom: 10px;
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
