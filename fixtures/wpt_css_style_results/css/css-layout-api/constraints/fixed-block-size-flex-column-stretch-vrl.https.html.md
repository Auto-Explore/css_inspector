# css/css-layout-api/constraints/fixed-block-size-flex-column-stretch-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-column-stretch-vrl.https.html"
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
  writing-mode: vertical-rl;
  background: red;
  --expected-block-size: 90; /* flex-item should stretch to (100 - 10)px */
  margin-left: 10px;
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
