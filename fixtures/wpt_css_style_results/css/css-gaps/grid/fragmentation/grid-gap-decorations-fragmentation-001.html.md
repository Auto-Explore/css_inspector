# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-001.html"
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
    grid-template-rows: repeat(3, 80px);
    row-gap: 30px;
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
    height: 20px;
    left: 0;
    top: 80px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
