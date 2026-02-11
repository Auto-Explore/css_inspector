# css/css-grid/grid-items/percentage-size-replaced-subitems-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/percentage-size-replaced-subitems-001.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  border: solid 5px black;
  grid: 100px / 150px;
  margin: 10px;
  vertical-align: top;
}

.item {
  overflow: scroll;
  border: solid magenta;
  border-width: 12px 9px 6px 3px;
  margin: 1px 2px 3px 4px;
  padding: 5px 15px 10px 20px;
  background: cyan;
}

img {
  width: 100%;
  height: 100%;
  display: block;
}

.horizontalTB { writing-mode: horizontal-tb; }
.verticalLR { writing-mode: vertical-lr; }
.verticalRL {  writing-mode: vertical-rl; }
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
      "message": "Invalid value for property “border”.",
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
