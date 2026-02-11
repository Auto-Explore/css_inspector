# css/css-grid/placement/grid-container-change-grid-tracks-recompute-child-positions-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-container-change-grid-tracks-recompute-child-positions-001.html"
}
```

## style[0]

```css

.grid {
    grid-auto-flow: row dense;
    grid-auto-rows: 5px;
    grid-auto-columns: 5px;
}
#firstGridItem {
    grid-row: auto;
    grid-column: 1;
}

#secondGridItem {
    grid-row: 1;
    grid-column: auto;
}

#thirdGridItem {
    grid-row: auto;
    grid-column: auto;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-auto-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
