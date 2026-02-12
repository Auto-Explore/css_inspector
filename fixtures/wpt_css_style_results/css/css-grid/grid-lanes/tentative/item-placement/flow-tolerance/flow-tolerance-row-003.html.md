# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-003.html"
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
      flow-tolerance: 51px; /* Tracks within 51px are considered equal */
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
