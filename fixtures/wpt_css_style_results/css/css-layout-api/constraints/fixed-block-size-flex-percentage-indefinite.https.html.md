# css/css-layout-api/constraints/fixed-block-size-flex-percentage-indefinite.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-flex-percentage-indefinite.https.html"
}
```

## style[0]

```css

body {
  display: flex;
}

.test {
  background: red;
  --expected-block-size: null; /* Percentage which we are resolving height against is indefinite. */
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
