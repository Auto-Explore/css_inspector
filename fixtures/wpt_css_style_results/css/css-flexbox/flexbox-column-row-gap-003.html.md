# css/css-flexbox/flexbox-column-row-gap-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-column-row-gap-003.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      border: 1px solid black;
      column-gap: 20px;
      row-gap: 40px;
      align-content: space-around;
      justify-content: space-around;
    }
    .rowWrap {
      flex-flow: row wrap;
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
    .w200 {
      width: 200px;
    }
    .hclamp {
      max-height: 90px;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
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
