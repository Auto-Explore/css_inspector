# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-002.html"
}
```

## style[0]

```css

  .multicol {
    columns: 3;
    column-fill: auto;
    width: 100px;
    height: 100px;
    background: green;
  }
  .grid-container {
    display: grid;
    grid-template-rows: repeat(3, 100px);
    row-gap: 100px;
    background: red;
    row-rule: solid;
  }
  .grid-item {
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
