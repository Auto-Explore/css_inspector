# css/css-gaps/grid/subgrid/subgrid-gap-decorations-003.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-003.html"
}
```

## style[0]

```css

  .grid-container {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
    gap: 10px;
    width: 100px;
    height: 100px;
  }
  .subgrid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: subgrid;

    grid-column: 1 / -1;
    grid-row: 1 / -1;
    column-gap: 20px;
    column-rule: green solid 20px;
    row-rule: green solid 10px;

    background: red;
  }
  .subgrid-item {
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
