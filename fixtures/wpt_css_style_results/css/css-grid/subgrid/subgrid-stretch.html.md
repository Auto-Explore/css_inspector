# css/css-grid/subgrid/subgrid-stretch.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/subgrid-stretch.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  .grid {
    display: inline-grid;
    grid-template-columns: 100px;
    grid-template-rows: 100px;
  }
  .subgrid {
    display: grid;
    background-color: blue;
  }
  .rows {
    grid-template-rows: subgrid;
    align-self: baseline;
  }
  .columns {
    grid-template-columns: subgrid;
    justify-self: baseline;
  }
  .vrl {
    writing-mode: vertical-rl;
  }
  .vrl.rows {
    align-self: initial;
    justify-self: baseline;
  }
  .vrl.columns {
    justify-self: initial;
    align-self: baseline;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
