# css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-positioned-item-dynamic-change.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/abspos/row-grid-lanes-positioned-item-dynamic-change.html"
}
```

## style[0]

```css

    .grid-lanes {
        display: grid-lanes;
        grid-lanes-direction: row;
        grid-template-rows: 75px 25px;
        position: relative;
        width: 100px;
    }

    .absolute {
        background: green;
        position: absolute;
        grid-row: 1 / 2;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
