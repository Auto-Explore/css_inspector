# css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-cyclic-dependency-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-cyclic-dependency-003-ref.html"
}
```

## style[0]

```css

    .inline-grid {
      display: inline-grid;
      background-color: grey;
      position: relative;
      border: solid;
      text-orientation: sideways;
      font: 15px/1 Ahem;
      align-items: baseline;
    }

    .columns { grid-template-columns: 100px 100px; }
    .fit-content-rows { grid-template-rows: fit-content; }

    .height25  { height: 25px; }
    .height50  { height: 50px; }
    .height200Percent { height: 200%; }

    .firstRowFirstColumn {
      background-color: blue;
      grid-column: 1;
      grid-row: 1;
    }

    .firstRowSecondColumn {
      background-color: lime;
      grid-column: 2;
      grid-row: 1;
    }

    .autoRowAutoColumnSpanning2 {
      background-color: maroon;
      grid-column: span 2;
      grid-row: 2;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-template-rows”.",
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
