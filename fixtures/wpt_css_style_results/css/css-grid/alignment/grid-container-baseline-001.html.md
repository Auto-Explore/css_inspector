# css/css-grid/alignment/grid-container-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-container-baseline-001.html"
}
```

## style[0]

```css

.wrapper {
  position: relative;
  margin: 10px;
  line-height: 0;
}

.grid {
  display: inline-grid;
  grid-auto-flow: column;
  background: grey;
}

.i1 {
  width: 150px;
  height: 100px;
  background: magenta;
}

.i2 {
  align-self: baseline;
  width: 75px;
  height: 50px;
  background: cyan;
}

.i3 {
  width: 100px;
  height: 75px;
  background: yellow;
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
      "message": "Invalid value for property “background”.",
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
