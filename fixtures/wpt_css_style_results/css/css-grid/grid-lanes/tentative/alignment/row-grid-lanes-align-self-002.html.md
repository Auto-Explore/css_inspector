# css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-002.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
    }

    .grid-lanes {
      display: grid-lanes;
      gap: 2px;
      grid-template-rows: repeat(3, 40px);
      grid-lanes-direction: row;
      color: #444;
      border: 1px solid;
      padding: 2px;
      width: 180px;
      margin: 5px;
      vertical-align: top;
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

    .self-start {
      align-self: self-start;
      background-color: darkred;
    }

    .self-end {
      align-self: self-end;
      background-color: darkblue;
    }

    .normal {
      align-self: normal;
      background-color: brown;
    }

    .short {
      height: 15px;
    }
  
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
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
