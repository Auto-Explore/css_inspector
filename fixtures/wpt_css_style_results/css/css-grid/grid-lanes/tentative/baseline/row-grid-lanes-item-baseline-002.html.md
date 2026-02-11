# css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/baseline/row-grid-lanes-item-baseline-002.html"
}
```

## style[0]

```css

    .grid-lanes {
      width: 500px;
      border: solid 5px;
      gap: 3px;
      display: grid-lanes;
      grid-lanes-direction: row;
      grid-template-rows: repeat(3, 100px);
      background-color: #f0f0f0;
      border-color: slateblue;
      align-items: center;
      margin-bottom: 20px;
      font: 16px/1 monospace;
    }

    .item {
      width: 100px;
      padding: 5px;
    }

    #item1 {
      background-color: #8cffa0;
      min-height: 30px;
      grid-row: 1 / 2;
      align-self: baseline;
      font-size: 16px;
    }

    #item2 {
      background-color: #a0c8ff;
      min-height: 50px;
      grid-row: 1 / 2;
      align-self: baseline;
      font-size: 24px;
    }

    #item3 {
      background-color: #ffa08c;
      min-height: 40px;
      grid-row: 1 / 2;
      align-self: center;
    }

    #item4 {
      background-color: #ffff8c;
      min-height: 60px;
      grid-row: 1 / 2;
      align-self: baseline;
      font-size: 12px;
    }

    #item5 {
      background-color: #ff8cff;
      min-height: 70px;
      grid-row: 2 / 3;
      align-self: last baseline;
    }

    #item6 {
      background-color: #8cffff;
      min-height: 50px;
      grid-row: 2 / 3;
      align-self: last baseline;
      font-size: 20px;
    }

    #item7 {
      background-color: #ff8c8c;
      min-height: 45px;
      grid-row: 2 / 3;
      align-self: start;
    }

    #item8 {
      background-color: #c8ffa0;
      min-height: 35px;
      grid-row: 3 / 4;
      align-self: baseline;
    }

    #item9 {
      background-color: #a0a0ff;
      min-height: 55px;
      grid-row: 3 / 4;
      align-self: baseline;
      font-size: 28px;
    }

    #item10 {
      background-color: #ffc88c;
      min-height: 50px;
      grid-row: 3 / 4;
      align-self: end;
    }
  
```

```json
{
  "errors": 14,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
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
