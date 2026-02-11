# css/css-flexbox/flexbox-column-row-gap-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-column-row-gap-001.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      width: 200px;
      height: 220px;
      border: 1px solid black;
      column-gap: 10%;
      row-gap: 40px;
      align-content: space-around;
      justify-content: space-around;
      float: left;
    }
    .rowWrap {
      flex-flow: row wrap;
    }
    .columnWrap {
      flex-flow: column wrap;
    }
    .item {
      border: 1px solid blue;
      background: lightblue;
      width: 28px;
      height: 28px;
    }
    .autoLBMargins {
      margin-left: auto;
      margin-bottom: auto;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
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
    }
  ],
  "warnings": 0
}
```
