# css/css-grid/subgrid/alignment-in-subgridded-axes-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/alignment-in-subgridded-axes-001.html"
}
```

## style[0]

```css

body {
  margin: 0;
  padding: 0;
  font-size: 0;
}

.grid {
  background: gray;
  display: inline-grid;
  grid-auto-rows: 100px;
  grid-template-columns: 100px;
}

.subgrid {
  margin: 10px;
  padding: 10px;
  display: grid;
  background: orangered;
  border: blue solid 10px;
  grid-template: subgrid / subgrid;
}

.as { align-self: start; }
.ae { align-self: end; }
.ac { align-self: center; }
.ab { align-self: baseline; }

.js { justify-self: start; }
.je { justify-self: end; }
.jc { justify-self: center; }
.jb { justify-self: baseline; }
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
