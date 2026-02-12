# css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-017.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/fragmentation/grid-gap-decorations-fragmentation-017.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .multi-col {
    height: 100px;
    width: 320px;
    columns: 3;
    column-fill: auto;
    column-gap: 10px;
    background: lightgray;

  }
  .grid-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(4, 90px);
    column-gap: 10px;
    row-gap: 20px;
    row-rule: solid 10px red;
    column-rule: solid 6px blue;
  }
  .grid-container>div {
    background-color: skyblue;
  }
  .margin-top {
    margin-top: 10px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
