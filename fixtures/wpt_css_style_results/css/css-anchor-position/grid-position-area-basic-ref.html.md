# css/css-anchor-position/grid-position-area-basic-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/grid-position-area-basic-ref.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  grid: 150px 100px / 200px 300px;
  padding: 20px;
  position: relative;
  border: 1px solid;
}

#positioned {
  position: absolute;
  background: magenta;
  grid-column: 1 / 2;
  grid-row: 1 / 2;
  left: 500px;
  bottom: 0;
}

#static {
  background: pink;
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}

#anchor {
  background: lime;
  grid-column: 2 / 3;
  grid-row: 2 / 3;
}

.abs-cb {
  width: 600px;
  height: 600px;
  position: relative;
}
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
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
      "message": "Invalid value for property “grid-row”.",
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
    }
  ],
  "warnings": 0
}
```
