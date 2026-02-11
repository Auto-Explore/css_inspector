# css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-alignment-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-alignment-ref.html"
}
```

## style[0]

```css

    html,body {
        color:black;
        background-color:white;
        font:15px/1 monospace;
    }

    .container {
        width: 800px;
        height: 850px;
        border: 2px solid black;
        margin: 20px;
    }

    .grid {
        display: grid;
        grid-template-rows: repeat(5, 150px);
        grid-auto-columns: auto;
        position: relative;
        padding: 20px;
        gap: 15px;
        border: 1px dashed #999;
        width: 700px;
    }

    .item {
        background: lightblue;
        padding: 10px;
        border: 1px solid blue;
        width: 80px;
    }

    .abspos {
        position: absolute;
        width: 40px;
        height: 60px;
        border: 2px solid black;
        font-size: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    .align-self {
        grid-row: 2 / 3;
    }

    .align-start {
        align-self: start;
        background: lightcoral;
    }

    .align-end {
        align-self: end;
        background: lightgreen;
    }

    .align-center {
        align-self: center;
        background: lightblue;
    }

    .justify-self {
        grid-row: 4 / 5;
    }

    .justify-start {
        justify-self: start;
        background: gold;
    }

    .justify-end {
        justify-self: end;
        background: silver;
    }

    .justify-center {
        justify-self: center;
        background: tan;
    }

    .combo-center-center {
        grid-row: 3 / 4;
        grid-column: 1 / 2;
        align-self: center;
        justify-self: center;
        background: hotpink;
    }

    .safe-align {
        position: absolute;
        background: orange;
        border: 1px solid darkorange;
        grid-row: 1 / 2;
        height: 170px;
        width: 30px;
        align-self: safe end;
        justify-self: center;
        font-size: 10px;
    }

    .unsafe-align {
        position: absolute;
        background: purple;
        border: 1px solid darkmagenta;
        grid-row: 5 / 6;
        height: 170px;
        width: 30px;
        align-self: unsafe end;
        justify-self: center;
        font-size: 10px;
    }
  
```

```json
{
  "errors": 18,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid value for property “background”.",
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
