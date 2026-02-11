# css/css-layout-api/child-constraints/available-inline-size-htb-htb.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/child-constraints/available-inline-size-htb-htb.https.html"
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
  line-height: 0;

  --available-inline-size: 20;
}

.inline {
  display: inline-block;
  height: 8px;
}

.inline-size-10 { width: 10px; }
.inline-size-30 { width: 30px; }

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
