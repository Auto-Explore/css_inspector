# css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/subgrid/track-sizing/grid-lanes-subgrid.html"
}
```

## style[0]

```css

    grid {
      display: inline-grid-lanes;
      grid-template-columns: 50px 100px 200px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
