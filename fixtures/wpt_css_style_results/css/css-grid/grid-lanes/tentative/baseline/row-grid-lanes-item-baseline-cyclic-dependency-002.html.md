# css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-cyclic-dependency-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-cyclic-dependency-002.html"
}
```

## style[0]

```css

    .grid-lanes {
      display: inline-grid-lanes;
      grid-lanes-direction: row;
      background-color: grey;
      position: relative;
      border: solid;
      text-orientation: sideways;
      font: 15px/1 Ahem;
      align-items: baseline;
    }

    .flex-rows        { grid-template-rows: 1fr; }
    .max-flex-rows    { grid-template-rows: minmax(0px, 1fr); }

    .height25  { height: 25px; }
    .height50  { height: 50px; }
    .height200Percent { height: 200%; }

    .firstRowFirstItem {
      background-color: blue;
      grid-row: 1;
      width: 100px;
    }

    .firstRowSecondItem {
      background-color: lime;
      grid-row: 1;
      width: 100px;
    }

    .secondRowSpanning2 {
      background-color: maroon;
      width: 200px;
      grid-row: 2;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
