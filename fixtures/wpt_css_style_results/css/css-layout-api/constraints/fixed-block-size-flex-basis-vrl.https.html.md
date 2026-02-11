# css/css-layout-api/constraints/fixed-block-size-flex-basis-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-basis-vrl.https.html"
}
```

## style[0]

```css

body {
  display: flex;
}

.test {
  writing-mode: vertical-rl;
  background: red;
  --expected-block-size: 100;
  flex-basis: 100px;
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
