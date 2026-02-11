# css/css-layout-api/child-constraints/available-block-size-invalid.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/available-block-size-invalid.https.html"
}
```

## style[0]

```css

.test {
  writing-mode: horizontal-tb;
  background: red;
  width: 100px;
}

.child {
  writing-mode: vertical-rl;
  visibility: hidden;
  line-height: 0;

  --available-block-size: -20;
}

.inline {
  display: inline-block;
  width: 8px;
}

.inline-size-10 { height: 10px; }

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
