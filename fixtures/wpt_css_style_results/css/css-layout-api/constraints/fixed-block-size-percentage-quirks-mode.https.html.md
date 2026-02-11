# css/css-layout-api/constraints/fixed-block-size-percentage-quirks-mode.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-percentage-quirks-mode.https.html"
}
```

## style[0]

```css

body {
  height: 100px;
}

.test {
  background: red;
  --expected-block-size: 50; /* In quirks mode we should get 100px * 50% */
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
