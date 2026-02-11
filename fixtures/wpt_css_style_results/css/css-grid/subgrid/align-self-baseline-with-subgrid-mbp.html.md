# css/css-grid/subgrid/align-self-baseline-with-subgrid-mbp.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/align-self-baseline-with-subgrid-mbp.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white;  padding:0; margin:0;
}
.grid {
    font:16px/1 Ahem;
    display: inline-grid;
    border: 1px solid black;
    grid-template-columns: auto auto;
}
.subgrid {
    display: grid;
    grid-template-rows: subgrid;
    border: 9px solid blue;
    margin-top: 15px;
    padding-top: 6px;
}
.first-baseline {
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
