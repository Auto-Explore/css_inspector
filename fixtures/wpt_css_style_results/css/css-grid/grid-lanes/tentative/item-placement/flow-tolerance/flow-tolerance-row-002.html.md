# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-002.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
    }

    grid {
      display: inline-grid-lanes;
      grid-lanes-direction: row;
      grid-auto-flow: column;
      gap: 10px;
      grid-template-rows: repeat(3, 100px);
      flow-tolerance: normal; /* Should resolve to 1em (16px) */
      color: #444;
      border: 1px solid;
      padding: 2px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 10px;
      margin: 3px;
      border: 2px solid blue;
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
      "message": "Unknown property “flow-tolerance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
