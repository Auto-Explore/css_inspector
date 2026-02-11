# css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-002-ref.html"
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
      grid-template-rows: repeat(3, 40px);
      grid-template-columns: repeat(2, min-content);
      color: #444;
      border: 1px solid;
      padding: 2px;
      width: 180px;
      margin: 5px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 2px;
      width: 45px;
    }

    .safe-end {
      align-self: safe end;
      background-color: purple;
      height: 50px;
    }

    .safe-center {
      align-self: safe center;
      background-color: teal;
      height: 45px;
    }

    .unsafe-end {
      align-self: end;
      background-color: darkmagenta;
      height: 50px;
    }

    .unsafe-center {
      align-self: center;
      background-color: darkcyan;
      height: 45px;
    }

    .normal {
      align-self: normal;
      background-color: brown;
    }

    .self-start {
      align-self: self-start;
      background-color: darkred;
    }

    .self-end {
      align-self: self-end;
      background-color: darkblue;
    }

    .short {
      height: 15px;
    }
  
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
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
