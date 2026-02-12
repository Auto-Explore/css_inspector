# css/filter-effects/filter-function/filter-function-006.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-006.html"
}
```

## style[0]

```css

.test {
    position: relative;
    width: 50px;
    height: 100px;
    box-sizing: border-box;
    border: 25px solid black;
    image-rendering: crisp-edges;
    border-image: filter(url(resources/green-transparent-80x80.png), drop-shadow(50px 0 0 green)) 30 / 75px / 50px;
}
.red {
    position: absolute;
    left: -25px;
    top: -25px;
    width: 100px;
    height: 100px;
    background: red;
    z-index: -1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
