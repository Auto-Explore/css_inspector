# css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid-intrinsic-sizing-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid-intrinsic-sizing-ref.html"
}
```

## style[0]

```css

    grid {
      display: inline-grid;
      grid-template-columns: auto min-content max-content;
      border: 1px solid;
      background: lightgrey;
    }

    subgrid {
      display: grid;
      grid: subgrid / subgrid;
      grid-column: 2 / span 2;
      grid-row: 1 / span 2;
      background: lightblue;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
