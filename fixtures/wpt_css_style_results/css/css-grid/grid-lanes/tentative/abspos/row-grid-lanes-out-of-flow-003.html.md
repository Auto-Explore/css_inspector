# css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-out-of-flow-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-out-of-flow-003.html"
}
```

## style[0]

```css

    .container {
        width: 400px;
        height: 500px;
        position: relative;
        border: 2px solid black;
        margin: 20px;
    }

    .grid-lanes {
        display: grid-lanes;
        grid-lanes-direction: row;
        grid-template-rows: 50px 100px 150px;
        border: 5px solid blue;
        margin: 30px 20px;
        padding: 15px 5px;
        gap: 10px;
    }

    .grid-lanes-item {
        background: lightgray;
        padding: 10px;
        width: 200px;
    }

    .nested-container {
        background: lightgreen;
        padding: 5px;
        border: 1px solid green;
    }

    .absolute {
        position: absolute;
        background: red;
        width: 60px;
        height: 40px;
        top: 50px;
        left: 100px;
        border: 1px solid darkred;
    }

    .static-pos {
        position: absolute;
        background: orange;
        width: 10px;
        height: 10px;
    }
  
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
