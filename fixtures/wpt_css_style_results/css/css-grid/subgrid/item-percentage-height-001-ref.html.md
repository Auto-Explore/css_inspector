# css/css-grid/subgrid/item-percentage-height-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/item-percentage-height-001-ref.html"
}
```

## style[0]

```css


.wrapper {
  padding: 2px;
  padding-top: 5px;
  border: 1px solid;
}
.grid {
  display: grid;
  grid: auto auto / auto;
  grid-row-gap: 60px;
  height: 300px;
  background: lightgrey;
}

.subgrid {
  display: grid;
  grid: subgrid / auto;
  grid-row: span 2;
  grid-row-gap: 40px;
  padding: 10px;
  padding-top: 5px;
}

.item {
  align-self: start;
  width: 100px;
  background: grey;
  margin-left: 10px;
}

.hidden { visibility: hidden; }
.a1 { grid-area: 1/1; }
.a2 { grid-area: 2/1; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
