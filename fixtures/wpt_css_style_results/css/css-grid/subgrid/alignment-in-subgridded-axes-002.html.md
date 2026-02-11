# css/css-grid/subgrid/alignment-in-subgridded-axes-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/alignment-in-subgridded-axes-002.html"
}
```

## style[0]

```css

div {
  display: inline-grid;
  gap: 5px;
}

.grid { grid-template: 50px / 50px }
.vlr { writing-mode: vertical-lr }

.subgrid {
  background: gray;
  grid-template: subgrid / subgrid;
}

.item {
  background: orange;
  height: 20px;
  width: 20px;
}

.as { align-self: start }
.ae { align-self: end }
.ac { align-self: center }
.ab { align-self: baseline }

.js { justify-self: start }
.je { justify-self: end }
.jc { justify-self: center }
.jb { justify-self: baseline }
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
