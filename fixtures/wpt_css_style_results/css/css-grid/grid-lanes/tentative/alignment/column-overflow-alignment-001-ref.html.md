# css/css-grid/grid-lanes/tentative/alignment/column-overflow-alignment-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-overflow-alignment-001-ref.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    background: gray;
    position: relative;
    grid-template-columns: 50px;
    justify-items: unsafe center;
    width: 50px;
}
.overflow-safe {
    justify-self: safe end;
    width: 75px;
    height: 50px;
    background-color: lightgreen;
}
.overflow-unsafe {
    justify-self: unsafe flex-end;
    width: 75px;
    height: 50px;
    background-color: lightblue;
}
.small-center-item {
    width: 25px;
    background-color: lightyellow;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
