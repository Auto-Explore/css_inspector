# css/css-sizing/dynamic-change-inline-size-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/dynamic-change-inline-size-003-ref.html"
}
```

## style[0]

```css

body {
  inline-size: 200px;
}

.abs {
  position: absolute;
  background: green;
  inline-size: 100px;
  padding-top: 100px;
}
.layout {
  font: 50px/1 Ahem;
  color: red;
  background-color: red;
  width: 100px;
  padding-top: 25%; /* This resolves against body's inline-size. */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
