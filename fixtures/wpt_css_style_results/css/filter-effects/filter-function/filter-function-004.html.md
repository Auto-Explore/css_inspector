# css/filter-effects/filter-function/filter-function-004.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-004.html"
}
```

## style[0]

```css

.test {
    position: relative;
    width: 100px;
    height: 100px;
    mask: filter(url(resources/green-transparent-100x100.png), drop-shadow(50px 0 0 black)) top left no-repeat alpha;
    -webkit-mask: filter(url(resources/green-transparent-100x100.png), drop-shadow(50px 0 0 black)) top left no-repeat alpha;
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
