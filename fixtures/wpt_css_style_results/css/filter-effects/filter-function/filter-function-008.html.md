# css/filter-effects/filter-function/filter-function-008.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-008.html"
}
```

## style[0]

```css

.test {
    position: relative;
    width: 100px;
    height: 100px;
    mask: filter(url(resources/green-transparent-20x10.png), drop-shadow(50px 0 0 black)) top left / 100px 100px no-repeat alpha;
    -webkit-mask: filter(url(resources/green-transparent-20x10.png), drop-shadow(50px 0 0 black)) top left / 100px 100px no-repeat alpha;
}
.content {
    width: 100px;
    height: 100px;
    background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “mask”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-mask”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
