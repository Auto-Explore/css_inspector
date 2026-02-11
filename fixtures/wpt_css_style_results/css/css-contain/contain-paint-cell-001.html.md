# css/css-contain/contain-paint-cell-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-cell-001.html"
}
```

## style[0]

```css

  div#table
    {
      display: table;
      font-family: monospace;
      font-size: 100px;
      table-layout: fixed;
      width: 4ch;
    }

  div.column
    {
      display: table-column;
    }

  div#middle-column
    {
      width: 4ch;
    }

  div.row
    {
      display: table-row;
    }

  div.cell
    {
      background-color: white;
      display: table-cell;
    }

  div#contain
    {
      color: green;
      contain: paint;
    }

  span
    {
      background-color: red;
      color: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
