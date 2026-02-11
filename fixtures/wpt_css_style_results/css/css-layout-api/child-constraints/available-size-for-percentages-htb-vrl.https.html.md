# css/css-layout-api/child-constraints/available-size-for-percentages-htb-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/available-size-for-percentages-htb-vrl.https.html"
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

  --available-inline-size: 50;
  --available-block-size: 20;

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
