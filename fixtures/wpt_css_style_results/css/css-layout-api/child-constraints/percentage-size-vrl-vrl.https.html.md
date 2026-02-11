# css/css-layout-api/child-constraints/percentage-size-vrl-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/percentage-size-vrl-vrl.https.html"
}
```

## style[0]

```css

.test {
  writing-mode: vertical-rl;
  background: red;
  height: 100px;
}

.child {
  writing-mode: vertical-rl;
  visibility: hidden;
  height: 10px;

  --percentage-block-size: 20;

  --inline-size-expected: 10px;
  --block-size-expected: 10px;
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
