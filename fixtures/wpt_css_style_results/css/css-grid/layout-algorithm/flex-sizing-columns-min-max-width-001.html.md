# css/css-grid/layout-algorithm/flex-sizing-columns-min-max-width-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/flex-sizing-columns-min-max-width-001.html"
}
```

## style[0]

```css

.grid {
   margin: 3px;
   grid: 50px / minmax(10px, 1fr) minmax(10px, 4fr);
   grid-column-gap: 33px;
   border: 5px dashed;
   padding: 2px;
}

.float { float: left; }

.item:nth-child(1) { background-color: purple; }
.item:nth-child(2) { background-color: blue; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
