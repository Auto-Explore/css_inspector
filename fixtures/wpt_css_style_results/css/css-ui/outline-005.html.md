# css/css-ui/outline-005.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-005.html"
}
```

## style[0]

```css

body > div {
  width: 100px;
  height: 100px;
  overflow: hidden;
  background: red;
  display: table-cell; /* Make a BFC */
}
div > div {
  width: 50px;
  height: 50px;
  margin: 25px;
  background: green;
  border-radius: 100%;
  outline: solid green 100px;
  outline-offset: -1px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
