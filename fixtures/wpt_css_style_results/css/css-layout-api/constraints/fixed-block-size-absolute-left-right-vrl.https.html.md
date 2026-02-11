# css/css-layout-api/constraints/fixed-block-size-absolute-left-right-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-block-size-absolute-left-right-vrl.https.html"
}
```

## style[0]

```css

body {
  position: relative;
  width: 120px;
}

.test {
  writing-mode: vertical-rl;
  background: red;
  position: absolute;
  left: 0px;
  right: 20px;
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
