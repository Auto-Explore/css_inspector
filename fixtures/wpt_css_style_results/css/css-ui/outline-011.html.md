# css/css-ui/outline-011.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-011.html"
}
```

## style[0]

```css

body > div {
  width: 100px;
  height: 100px;
  background: red;
  color: red;
  display: table-cell; /* Make a BFC */
}
div > div {
  width: 0;
  height: 0;
  margin: 50px;
  color: green;
  outline: solid currentcolor 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
