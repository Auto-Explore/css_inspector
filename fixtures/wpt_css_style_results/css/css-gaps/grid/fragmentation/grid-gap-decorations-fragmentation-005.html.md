# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-005.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-005.html"
}
```

## style[0]

```css

  .multicol {
    position: relative;
    columns: 3;
    column-fill: auto;
    width: 100px;
    height: 100px;
    background: green;
    column-gap: 0px;
  }
  .grid-container {
    display: grid;
    grid-template-rows: repeat(3, 98px);
    row-gap: 20px;
    background: red;
    row-rule: solid;
  }
  .grid-item {
    background: green;
  }
  .abspos {
    background: green;
    position: absolute;
    width: 66.66px;
    height: 2px;
    left: 0;
    top: 98px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
