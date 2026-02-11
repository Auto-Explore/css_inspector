# css/css-layout-api/child-constraints/percentage-size-invalid.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/percentage-size-invalid.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  width: 100px;
}

.child {
  visibility: hidden;
  width: 10px;
  line-height: 0;

  --percentage-block-size: -10;
}

.inline {
  display: inline-block;
  width: 10px;
  height: 10px;
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
