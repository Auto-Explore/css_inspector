# css/css-ui/outline-019.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-019.html"
}
```

## style[0]

```css

body > div {
  color: red;
  outline-color: currentcolor;
  display: table-cell; /*BFC*/
}

div > div {
  width: 0; height: 0;
  margin-left: 50px;
  margin-top: 50px;
  color: green;
  outline-color: inherit;
  outline-width: 50px;
  outline-style: solid;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
