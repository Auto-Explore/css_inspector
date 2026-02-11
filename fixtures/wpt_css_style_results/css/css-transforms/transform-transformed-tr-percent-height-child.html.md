# css/css-transforms/transform-transformed-tr-percent-height-child.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-transformed-tr-percent-height-child.html"
}
```

## style[0]

```css

  table, td, tr {
    margin: 0px;
    padding: 0px;
    border-spacing: 0px;
  }
  table {
    background-color: lightblue;
  }
  td {
    width: 50px;
    height: 50px;
    background-color: lightgrey;
  }
  .contblock {
    transform: translateX(10px);
    width: 200px;
    height: 200px;
    background-color: lightyellow;
  }
  .abspos {
    position: absolute;
    top: 20px;
    left: 20px;
    width: 100%;
    height: 100%;
    background-color: blue;
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
      "message": "Invalid value for property “background-color”.",
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
