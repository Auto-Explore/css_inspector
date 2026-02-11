# css/css-layout-api/layout-child/inflow.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/layout-child/inflow.https.html"
}
```

## style[0]

```css

.test {
  --child-expected: ["1", "2"];

  background: red;
  width: 100px;
}

.inflow-1 {
  visibility: hidden;
  --child: 1;
}

.inflow-2 {
  visibility: hidden;
  --child: 2;
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
