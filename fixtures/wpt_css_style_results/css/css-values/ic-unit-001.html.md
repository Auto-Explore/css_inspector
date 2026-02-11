# css/css-values/ic-unit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ic-unit-001.html"
}
```

## style[0]

```css

@font-face {
    font-family: IcTestFullWidth;
    src: url(resources/IcTestFullWidth.woff2);
}

span {
    font: 20px IcTestFullWidth;
    background: green;
    color: green;
    top: 0; bottom: 0;
    position: absolute;
}
div {
    font: 20px IcTestFullWidth;
    background: red;
    color: red;
    position: relative;
    height: 10ic;
    width: 5ic;
    float: left;
}

div + div {
    width: auto;
}

div + div span {
    width: 5ic;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
