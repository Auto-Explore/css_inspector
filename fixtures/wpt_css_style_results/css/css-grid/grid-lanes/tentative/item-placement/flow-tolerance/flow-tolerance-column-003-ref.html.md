# css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-column-003-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/item-placement/flow-tolerance/flow-tolerance-column-003-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
    }

    .outer-grid {
      display: inline-grid;
      grid-template-columns: repeat(3, auto);
      column-gap: 10px;
      row-gap: 0;
      color: #444;
      border: 1px solid;
      padding: 2px;
    }

    .column {
      display: grid;
      grid-template-rows: auto;
      gap: 10px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 10px;
      margin: 3px;
      border: 2px solid blue;
      width: 70px;
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
