# css/css-values/ch-unit-003.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-unit-003.html"
}
```

## style[0]

```css

span {
    background: green;
    color: green;
    left: 0; right: 0;
    position: absolute;
}
div {
    background: red;
    color: red;
    position: relative;
    height: 5ch;
    width: 10ch;
    writing-mode: vertical-rl;
    text-orientation: mixed;
}

div + div {
    height: auto;
}

div + div span {
    height: 5ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
