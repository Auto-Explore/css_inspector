# css/css-flexbox/flexbox-safe-overflow-position-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-safe-overflow-position-001.html"
}
```

## style[0]

```css

    .flex {
      display: flex;
      width: 85px;
      height: 65px;
      border: 1px solid black;
      align-content: safe flex-end;
      justify-content: safe center;
      align-items: safe center;
      float: left;
      clear: both;
      margin-top: 100px;
    }
    .rowNoWrap {
      flex-flow: row nowrap;
    }
    .columnNoWrap {
      flex-flow: column wrap;
    }
    .item {
      border: 1px solid blue;
      background: lightblue;
      width: 28px;
      height: 28px;
      flex-shrink: 0;
    }
    .bigItem {
      border: 1px solid blue;
      background: lightblue;
      width: 28px;
      height: 90px;
      flex-shrink: 0;
    }
  
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “align-content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “justify-content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-items”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
