# css/css-values/ch-unit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-unit-001.html"
}
```

## style[0]

```css

span {
    background: green;
    color: green;
    top: 0; bottom: 0;
    position: absolute;
}
div {
    background: red;
    color: red;
    position: relative;
    height: 10ch;
    width: 5ch;
    float: left;
}

div + div {
    width: auto;
}

div + div span {
    width: 5ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
