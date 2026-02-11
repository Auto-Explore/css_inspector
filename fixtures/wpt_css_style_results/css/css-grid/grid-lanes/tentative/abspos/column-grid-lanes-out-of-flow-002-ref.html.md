# css/css-grid/grid-lanes/tentative/abspos/column-grid-lanes-out-of-flow-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/column-grid-lanes-out-of-flow-002-ref.html"
}
```

## style[0]

```css

    .container {
        width: 300px;
        height: 150px;
        border: 2px solid black;
        margin: 20px;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(4, 60px);
        grid-template-rows: auto;
        border: 1px solid blue;
        padding: 10px;
        gap: 5px;
        position: relative;
    }

    .item {
        background: lightblue;
        padding: 5px;
        height: 100px;
    }

    .wrapper {
        background: lightgreen;
        padding: 5px;
        height: 60px;
        border: 1px dashed green;
    }

    .absolute {
        position: absolute;
        grid-column: 1 / 2;
        background: red;
        top: 30px;
        width: 20px;
        height: 20px;
    }

    .static-pos-with-grid-column {
        position: absolute;
        background: yellow;
        grid-column: 2 / 3;
        width: 10px;
        height: 10px;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
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
