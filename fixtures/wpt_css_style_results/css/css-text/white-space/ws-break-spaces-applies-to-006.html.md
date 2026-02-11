# css/css-text/white-space/ws-break-spaces-applies-to-006.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/ws-break-spaces-applies-to-006.html"
}
```

## style[0]

```css

  div#test, table#reference
    {
      border: black solid 2px;
      font-family: monospace;
      font-size: 32px;
      margin-bottom: 0.25em;
      width: 4ch;
    }

  div#test
    {
      display: table;
      white-space: break-spaces;
    }

  div#row
    {
      display: table-row;
    }

  div#cell
    {
      display: table-cell;
    }

  table#reference
    {
      border-spacing: 0px;
      white-space: pre;
    }

  td
    {
      padding: 0px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
