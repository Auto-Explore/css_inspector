# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-004.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-004.html"
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
    grid-template-rows: repeat(3, 90px);
    row-gap: 10px;
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
    height: 10px;
    left: 0;
    top: 90px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
