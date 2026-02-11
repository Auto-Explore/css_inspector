# css/css-layout-api/style-map-multi.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/style-map-multi.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  margin: 10px;
  width: 100px;

  /* Properties under test. */
  --foo:bar;
  margin-left: 2px;
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
