# css/css-layout-api/layout-child/before-after.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/layout-child/before-after.https.html"
}
```

## style[0]

```css

.test {
  --child-expected: ["1", "2", "3"];

  background: red;
  width: 100px;
}

.test::before {
  visibility: hidden;
  content: 'before';
  --child: 1;
}

.inflow {
  visibility: hidden;
  --child: 2;
}

.test::after {
  visibility: hidden;
  content: 'after';
  --child: 3;
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
