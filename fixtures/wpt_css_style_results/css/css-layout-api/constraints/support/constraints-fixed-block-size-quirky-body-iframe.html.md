# css/css-layout-api/constraints/support/constraints-fixed-block-size-quirky-body-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/support/constraints-fixed-block-size-quirky-body-iframe.html"
}
```

## style[0]

```css

body {
  margin: 0;
  --expected-block-size: 200;
}

.child {
  background: green;
}

@supports (display: layout(test)) {
  body {
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
