# css/css-grid/grid-items/grid-intrinsic-maximums-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-intrinsic-maximums-ref.html"
}
```

## style[0]

```css


.item {
    font: 10px/1 Ahem;
    background: cyan;
    grid-column: 1 / -1;
}

.abs {
    width: 100%;
    height: 5px;
    position: absolute;
}

.grid {
    border: 2px solid black;
    display: grid;
    position: relative;
    padding-top: 10px;
    margin-bottom: 10px;
    width: 100px;
    justify-items: start;
}

.float {
    float: left;
    width: 200px;
}

.col1 { grid-column: 1 / 2; background: orange; }
.col2 { grid-column: 2 / 3; background: indigo; }
.col3 { grid-column: 3 / 4; background: green; }

```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
