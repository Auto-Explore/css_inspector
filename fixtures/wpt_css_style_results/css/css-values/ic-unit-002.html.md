# css/css-values/ic-unit-002.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ic-unit-002.html"
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
    left: 0; right: 0;
    position: absolute;
}

div {
    font: 20px IcTestFullWidth;
    background: red;
    color: red;
    position: relative;
    height: 5ic;
    width: 10ic;
    writing-mode: vertical-rl;
    text-orientation: upright;
}

div + div {
    height: auto;
}

div + div span {
    height: 5ic;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
