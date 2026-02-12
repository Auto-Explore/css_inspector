# css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/alignment/row-grid-lanes-align-self-001-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:15px/1 monospace; padding:0; margin:0;
    }

    .grid {
      display: grid;
      gap: 2px;
      grid-template-rows: repeat(3, 40px);
      grid-template-columns: repeat(4, min-content);
      color: #444;
      border: 1px solid;
      padding: 2px;
      width: 250px;
      margin: 5px;
    }

    item {
      background-color: #444;
      color: #fff;
      padding: 2px;
      width: 45px;
    }

    .start {
      align-self: start;
      background-color: red;
    }

    .end {
      align-self: end;
      background-color: blue;
    }

    .center {
      align-self: center;
      background-color: green;
    }

    .stretch {
      align-self: stretch;
      background-color: orange;
    }

    .auto {
      align-self: auto;
      background-color: gray;
    }

    .short {
      height: 15px;
    }

    .medium {
      height: 20px;
    }

    .tall {
      height: 30px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
