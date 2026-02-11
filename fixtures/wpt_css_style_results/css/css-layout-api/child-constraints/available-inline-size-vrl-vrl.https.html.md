# css/css-layout-api/child-constraints/available-inline-size-vrl-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/available-inline-size-vrl-vrl.https.html"
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
  line-height: 0;

  --available-inline-size: 20;
}

.inline {
  display: inline-block;
  width: 8px;
}

.inline-size-10 { height: 10px; }
.inline-size-30 { height: 30px; }

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
