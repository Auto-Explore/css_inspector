# css/css-layout-api/edges/border-vrl.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/edges/border-vrl.https.html"
}
```

## style[0]

```css

.test {
  writing-mode: vertical-rl;
  background: red;
  box-sizing: border-box;
  height: 100px;

  --edges-inline-start-expected: 10;
  --edges-inline-end-expected: 0;
  --edges-block-start-expected: 8;
  --edges-block-end-expected: 20;

  font-size: 8px;

  border-color: transparent;
  border-style: solid;
  border-width: 10px 1em 0 20px;
}

@supports (display: layout(test)) {
  .test {
    display: layout(test);
    background: green;
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
