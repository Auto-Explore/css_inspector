# css/css-contain/contain-layout-cell-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-cell-001.html"
}
```

## style[0]

```css

  div#table
    {
      background-color: blue;
      border-spacing: 2px;
      display: table;
      height: 206px;
      table-layout: fixed;
      width: 206px;
    }

  div.row
    {
      display: table-row;
    }

  div.cell
    {
      background-color: white;
      display: table-cell;
      vertical-align: top;
    }

  div#contain
    {
      contain: layout;
    }

  span
    {
      background-color: red;
      color: yellow;
      font-family: monospace;
      vertical-align: top;
    }

  div#abs-pos
    {
      background-color: green;
      color: white;
      font-family: monospace;
      left: 0px;
      position: absolute;
      top: 0px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
