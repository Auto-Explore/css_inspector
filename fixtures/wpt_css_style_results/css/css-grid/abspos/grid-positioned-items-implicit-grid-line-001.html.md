# css/css-grid/abspos/grid-positioned-items-implicit-grid-line-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-items-implicit-grid-line-001.html"
}
```

## style[0]

```css


.grid {
  grid-template-columns: 100px 200px;
  grid-template-rows: 50px 150px;
  width: 500px;
  height: 300px;
  border: 5px solid black;
  margin: 30px;
  padding: 15px;
  /* Ensures that the grid container is the containing block of the absolutely positioned grid children. */
  position: relative;
}

.absolute {
  position: absolute;
}

.startImplicitLine {
  background-color: blue;
  grid-column: 5;
  grid-row: 8;
}

.endImplicitLine {
  background-color: orange;
  grid-column: 1 / 5;
  grid-row: 1 / 8;
}

.startImplicitLineSpan {
  background-color: blue;
  grid-column: span 5;
  grid-row: span 8;
}

.endImplicitLineSpan {
  background-color: orange;
  grid-column: 1 / span 5;
  grid-row: 1 / span 8;
}

```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
