# css/css-grid/placement/grid-container-change-named-grid-recompute-child-positions-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/placement/grid-container-change-named-grid-recompute-child-positions-001.html"
}
```

## style[0]

```css

.grid {
    grid-auto-flow: row dense;
}
#firstGridItem {
    grid-row: auto;
    grid-column: column;
}

#secondGridItem {
    grid-row: row;
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
