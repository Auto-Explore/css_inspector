# css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/column-grid-lanes-alignment-positioned-items-001-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:10px/1 monospace; padding:0; margin:0;
    }

    .grid {
      position: relative;
      display: grid;
      grid-template-columns: 100px 150px;
      grid-template-rows: 200px;
      width: 300px;
      background: grey;
      gap: 10px;
      padding: 10px;
    }

    .grid > div {
      position: absolute;
    }

    .grid > :nth-child(1) {
      grid-column: 1/2;
      align-self: start;
      background: green;
    }

    .grid > :nth-child(2) {
      grid-column: 2/3;
      align-self: start;
      background: blue;
    }

    .grid > :nth-child(3) {
      grid-column: 1/2;
      align-self: end;
      background: yellow;
    }

    .grid > :nth-child(4) {
      grid-column: 2/3;
      align-self: end;
      background: red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
