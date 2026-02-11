# css/css-layout-api/child-constraints/percentage-size-htb-htb.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/percentage-size-htb-htb.https.html"
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
  writing-mode: horizontal-tb;
  visibility: hidden;
  width: 10px;

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
