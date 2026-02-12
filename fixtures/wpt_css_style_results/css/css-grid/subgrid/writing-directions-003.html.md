# css/css-grid/subgrid/writing-directions-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/writing-directions-003.html"
}
```

## style[0]

```css


.grid {
  display: grid;
  grid-template: repeat(5, 10px) repeat(5, 10px) / repeat(5, 10px) repeat(5, 10px);
  width: 100px;
  height: 100px;
  border: 1px solid;
  background: grey;
}

.subgrid {
  position: relative;
  display: grid;
  grid-template: subgrid / subgrid;
  grid-column: span 10;
  grid-row: span 10;
}

.grid-item {
  width: 10px;
  height: 10px;
  background: orange;
  border: 1px solid;
  position: absolute;
}

.rtl { direction:rtl; }

.ltr { direction:ltr; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
