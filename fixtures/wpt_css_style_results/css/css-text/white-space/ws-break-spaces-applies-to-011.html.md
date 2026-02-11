# css/css-text/white-space/ws-break-spaces-applies-to-011.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/ws-break-spaces-applies-to-011.html"
}
```

## style[0]

```css

  div#cell, td#reference
    {
      border: black solid 2px;
      font-family: monospace;
      font-size: 32px;
      width: 4ch;
    }

  div#table
    {
      display: table;
    }

  div#test
    {
      display: table-row;
      white-space: break-spaces;
    }

  div#cell
    {
      display: table-cell;
      width: 4ch;
    }

  table
    {
      border-spacing: 0px;
      margin-top: 0.5em;
    }

  td#reference
    {
      padding: 0px;
      white-space: pre;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
