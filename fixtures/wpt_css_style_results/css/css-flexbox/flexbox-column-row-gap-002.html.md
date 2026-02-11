# css/css-flexbox/flexbox-column-row-gap-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-column-row-gap-002.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      width: 220px;
      height: 200px;
      border: 1px solid black;
      column-gap: 10%;
      row-gap: 10px;
      align-content: flex-start;
      justify-content: flex-end;
      float: left;
      writing-mode: vertical-lr;
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
