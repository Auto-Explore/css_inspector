# css/css-layout-api/intrinsic-sizes/absolute.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/intrinsic-sizes/absolute.https.html"
}
```

## style[0]

```css

.test {
  background: red;
  position: absolute;
}

@supports (display: layout(test-layout)) {
  .test {
    display: layout(test-layout);
    background: green;
  }
}

.container-1 {
  border: solid 2px;
  position: relative;
  height: 200px;
  width: 300px;
}

.container-2 {
  border: solid 2px;
  position: relative;
  height: 25px;
  width: 25px;
}

.horizontal {
  writing-mode: horizontal-tb;
}

.vertical {
  writing-mode: vertical-rl;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
