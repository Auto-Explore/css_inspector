# css/css-layout-api/style-map.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/style-map.https.html"
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
  }

  .test-0 { display: layout(test-0); }
  .test-1 { display: layout(test-1); }
  .test-2 { display: layout(test-2); }
  .test-3 { display: layout(test-3); }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
