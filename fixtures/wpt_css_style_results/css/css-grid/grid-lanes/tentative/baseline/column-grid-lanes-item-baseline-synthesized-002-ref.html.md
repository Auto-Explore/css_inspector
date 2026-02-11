# css/css-grid/grid-lanes/tentative/baseline/column-grid-lanes-item-baseline-synthesized-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/column-grid-lanes-item-baseline-synthesized-002-ref.html"
}
```

## style[0]

```css

    .container { position: relative; }
    .grid {
      display: grid;
      position: relative;
      text-orientation: sideways;
      grid: 100px 200px / 200px 100px;
      font-family: Ahem;
      line-height: 1;
      background-color: grey;
      justify-items: baseline;
      width: 300px;
    }
    .bigFont  { font-size: 50px; }
    .height25 { height: 25px; }
    .paddingLeft { padding-left: 25px; }
    .paddingRight { padding-right: 25px; }

    .firstRowFirstColumn {
      background-color: blue;
      grid-column: 1;
      grid-row: 1;
    }

    .secondRowFirstColumn {
      background-color: purple;
      grid-column: 1;
      grid-row: 2;
    }

    .autoRowSpanning2AutoColumn {
      background-color: aqua;
      grid-column: auto;
      grid-row: span 2;
    }

    .verticalRL {
      writing-mode: vertical-rl;
    }
    .verticalLR {
      writing-mode: vertical-lr;
    }
    .horizontalTB {
      writing-mode: horizontal-tb;
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
      "message": "Invalid value for property “background-color”.",
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
