# css/css-layout-api/constraints/fixed-inline-size-percentage-vlr.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/constraints/fixed-inline-size-percentage-vlr.https.html"
}
```

## style[0]

```css

body {
  height: 200px;
  writing-mode: vertical-lr;
}

.test {
  background: red;
  height: 50%;
}

@supports (display: layout(test)) {
  .test {
    background: green;
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
