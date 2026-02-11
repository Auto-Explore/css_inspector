# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-dynamic-update.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-dynamic-update.html"
}
```

## style[0]

```css

    html, body {
      color: black;
      background-color: white;
      padding: 0;
      margin: 0;
    }

    .grid-lanes {
      display: inline-grid-lanes;
      grid-template-rows: repeat(2, 100px);
      gap: 10px;
      flow-tolerance: normal;
      border: 1px solid;
      padding: 2px;
    }

    .item {
      background-color: #444;
      color: #fff;
      padding: 10px;
      margin: 3px;
      border: 2px solid blue;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
