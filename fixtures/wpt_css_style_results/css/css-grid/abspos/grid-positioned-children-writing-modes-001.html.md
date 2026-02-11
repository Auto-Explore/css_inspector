# css/css-grid/abspos/grid-positioned-children-writing-modes-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-children-writing-modes-001.html"
}
```

## style[0]

```css


.grid {
  margin: 5px;
  width: 100px;
  height: 75px;
  grid: 20px / 30px;
  padding: 5px 10px 15px 20px;
  border-style: solid;
  border-width: 5px 10px 15px 20px;
  float: left;
  /* Ensures that the grid container is the containing block of the grid children. */
  position: relative;
}

.absolute {
  position: absolute;
}

.onlyFirstRowOnlyFirstColumn {
  background-color: green;
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}

.offsets {
  left: 0;
  top: 0;
}

.red {
  background-color: red;
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
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
