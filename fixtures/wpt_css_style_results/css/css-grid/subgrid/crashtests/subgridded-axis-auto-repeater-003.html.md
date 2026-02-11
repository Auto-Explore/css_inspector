# css/css-grid/subgrid/crashtests/subgridded-axis-auto-repeater-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/crashtests/subgridded-axis-auto-repeater-003.html"
}
```

## style[0]

```css

  .grid {
    display: grid;
  }
  .subgrid {
    grid-template-columns: subgrid repeat(auto-fill, [x]);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
