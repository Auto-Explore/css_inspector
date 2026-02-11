# css/css-grid/grid-items/percentage-size-replaced-subitems-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/percentage-size-replaced-subitems-001-ref.html"
}
```

## style[0]

```css

.grid {
  display: inline-block;
  border: solid 5px black;
  width: 150px;
  height: 100px;
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
  width: calc(100% - 6px);
  height: calc(100% - 4px);
  box-sizing: border-box;
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
  "errors": 2,
  "messages": [
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
