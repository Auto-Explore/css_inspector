# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-row-004-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
    }

    .outer-grid {
      display: inline-grid;
      grid-template-rows: repeat(2, auto);
      row-gap: 10px;
      column-gap: 0;
      color: #444;
      border: 1px solid;
      padding: 2px;
    }

    .row {
      display: grid;
      grid-auto-flow: column;
      grid-auto-columns: auto;
      gap: 10px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 10px;
      margin: 3px;
      border: 2px solid blue;
      height: 70px;
      box-sizing: content-box;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
