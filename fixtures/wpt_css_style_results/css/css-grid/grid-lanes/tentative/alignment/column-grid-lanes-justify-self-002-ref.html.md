# css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-justify-self-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-justify-self-002-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
    }

    .grid {
      display: grid;
      gap: 2px;
      grid-template-columns: repeat(3, 70px);
      grid-template-rows: repeat(2, min-content);
      color: #444;
      border: 1px solid;
      padding: 2px;
      height: 150px;
      margin: 5px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 2px;
      height: 30px;
    }

    .safe-end {
      justify-self: safe end;
      background-color: purple;
      width: 80px;
    }

    .safe-center {
      justify-self: safe center;
      background-color: teal;
      width: 75px;
    }

    .unsafe-end {
      justify-self: end;
      background-color: darkmagenta;
      width: 80px;
    }

    .unsafe-center {
      justify-self: center;
      background-color: darkcyan;
      width: 75px;
    }

    .normal {
      justify-self: normal;
      background-color: brown;
    }

    .self-start {
      justify-self: self-start;
      background-color: darkred;
    }

    .self-end {
      justify-self: self-end;
      background-color: darkblue;
    }

    .narrow {
      width: 20px;
    }
  
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-self”.",
      "severity": "Error"
    },
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
