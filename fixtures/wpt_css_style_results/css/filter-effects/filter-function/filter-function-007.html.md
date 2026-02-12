# css/filter-effects/filter-function/filter-function-007.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-007.html"
}
```

## style[0]

```css

.test {
    position: relative;
    width: 100px;
    height: 100px;
    image-rendering: crisp-edges;
    background: filter(url(resources/green-transparent-20x10.png), drop-shadow(50px 0 0 green)) top left / 100px 100px no-repeat, red;
}
.red {
    position: absolute;
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
