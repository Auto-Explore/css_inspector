# css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid-flex.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid-flex.html"
}
```

## style[0]

```css

    grid {
      display: inline-grid-lanes;
      grid-template-columns: 1fr 2fr 3fr;
      border: 1px solid;
      background: lightgrey;
    }

    subgrid {
      display: grid;
      grid-template-columns: subgrid;
      grid-column: 2 / span 2;
      background: lightblue;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
