# css/css-grid/grid-lanes/tentative/abspos/column-grid-lanes-out-of-flow-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/column-grid-lanes-out-of-flow-003-ref.html"
}
```

## style[0]

```css

    .container {
        width: 500px;
        height: 400px;
        position: relative;
        border: 2px solid black;
        margin: 20px;
    }

    .grid {
        display: grid;
        grid-template-columns: 50px 100px 150px;
        grid-template-rows: auto;
        border: 5px solid blue;
        margin: 20px 30px;
        padding: 5px 15px;
        gap: 10px;
    }

    .grid-item {
        background: lightgray;
        padding: 10px;
        height: 200px;
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
  "errors": 4,
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
