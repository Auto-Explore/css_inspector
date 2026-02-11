# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-016-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-016-ref.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    height: 92px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;
  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(18, 10px) 4px 50px;;
    column-gap: 10px;
    background: teal;
  }

  .column-gap {
    position: absolute;
    top: 0px;
    height: 92px;
    background: blue;
    width: 6px;
  }
  .row-set {
    position: absolute;
    width: 100px;
    left: 0px;
    top: 12.5px;
    height: 205px;
    display: flex;
    flex-direction: column;
    row-gap: 15px;
  }
  .row-set>div {
    background: red;
    width: 100px;
    height: 5px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
