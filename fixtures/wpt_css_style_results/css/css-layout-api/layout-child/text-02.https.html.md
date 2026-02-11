# css/css-layout-api/layout-child/text-02.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/layout-child/text-02.https.html"
}
```

## style[0]

```css

/* We have a wrapper in this test to ensure that any text that is positioned
 * slightly outside the "test" box doesn't affect the rendering.
 * This wrapper has a 10px inline padding which does the trick. */
.wrapper {
  background: green;
  padding: 0 10px;
  width: 80px;
}

.test {
  --child-expected: ["default"];

  background: red;
  color: green;
  width: 80px;
  --child: default;
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
