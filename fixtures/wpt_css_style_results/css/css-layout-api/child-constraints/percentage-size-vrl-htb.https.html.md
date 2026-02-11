# css/css-layout-api/child-constraints/percentage-size-vrl-htb.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/percentage-size-vrl-htb.https.html"
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
  writing-mode: horizontal-tb;
  visibility: hidden;
  width: 10px;

  --percentage-inline-size: 20;

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
