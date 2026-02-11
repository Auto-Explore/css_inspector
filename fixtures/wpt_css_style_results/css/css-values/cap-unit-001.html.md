# css/css-values/cap-unit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/cap-unit-001.html"
}
```

## style[0]

```css

span {
    background: green;
    color: green;
    position: absolute;
}
div {
    font: 50px Ahem; /* cap-height of Ahem is 0.8em */
    background: red;
    position: relative;
    height: 180px;
    height: calc(180px - 2cap);  /* reduce to 100px if cap correctly supported */
    width: 100px;
}

div span {
    width: 2.5cap;
    height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
