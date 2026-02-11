# css/css-grid/grid-model/grid-size-shrink-to-fit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-model/grid-size-shrink-to-fit-001.html"
}
```

## style[0]

```css

.grid {
    display: grid;
    grid-template-columns: 200px 200px;
    grid-template-rows: 200px 200px;
}
.container {
    width: 600px;
    height: 600px;
}

#absolutePos {
   position: absolute;
}

#fixedPos {
   position: fixed;
}

#floatPos {
   float: left;
}

#one {
    color: blue;
    background: red;
    grid-column: 1;
    grid-row: 1;
}

#two {
    color: yellow;
    background: green;
    grid-column: 2;
    grid-row: 1;
}
#three {
    color: gray;
    background: pink;
    grid-column: 1;
    grid-row: 2;
}
#four {
    color: orange;
    background: brown;
    grid-column: 2;
    grid-row: 2;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
