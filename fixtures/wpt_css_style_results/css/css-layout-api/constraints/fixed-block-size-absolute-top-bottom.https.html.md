# css/css-layout-api/constraints/fixed-block-size-absolute-top-bottom.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-absolute-top-bottom.https.html"
}
```

## style[0]

```css

body {
  position: relative;
  height: 120px;
}

.test {
  background: red;
  position: absolute;
  top: 0px;
  bottom: 20px;
  --expected-block-size: 100;
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
