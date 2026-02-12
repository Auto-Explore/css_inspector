# css/css-grid/subgrid/item-percentage-height-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/item-percentage-height-001.html"
}
```

## style[0]

```css


.grid {
  display: grid;
  grid: auto auto / auto;
  grid-row-gap: 60px;
  border: 1px solid;
  padding: 2px;
  padding-top: 5px;
  height: 300px;
}

.subgrid {
  display: grid;
  grid: subgrid / auto;
  grid-row: span 2;
  grid-row-gap: 40px;
  background: lightgrey;
  padding: 10px;
  padding-top: 5px;
}

.item {
  align-self: start;
  height: 100%;
  width: 100px;
  background: grey;
}

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
