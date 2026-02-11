# css/css-flexbox/flexbox-dyn-resize-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-dyn-resize-001-ref.html"
}
```

## style[0]

```css

    .container {
      width: 100px;
      display: flex;
      border: 1px solid purple;
      margin-bottom: 15px;
    }
    .item {
       margin: 2px;
       background: lightblue;
    }
    .inline-box {
      display: inline-block;
      height: 10px;
      width: 10px;
      background: lightgray;
      border: 1px solid black;
     }
    #change-width {
      /* Using hardcoded CSS as reference for testcase's tweak: */
      width: 300px;
    }
    #change-flex {
      /* Using hardcoded CSS as reference for testcase's tweak: */
      flex: 0 0 75px;
    }
  
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
