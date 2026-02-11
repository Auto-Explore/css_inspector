# css/css-grid/subgrid/crashtests/contain-strict-subgrid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/crashtests/contain-strict-subgrid.html"
}
```

## style[0]

```css


#gridcontainer {
  display: grid;
  contain: strict;
}

.item {
  display: grid;
  grid: subgrid [x] / subgrid [x];
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
