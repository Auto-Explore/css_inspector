# css/css-text/white-space/ws-break-spaces-applies-to-015.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/ws-break-spaces-applies-to-015.html"
}
```

## style[0]

```css

  div#test, caption#reference
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
      display: table-caption;
      white-space: break-spaces;
    }

  table
    {
      margin-top: 0.5em;
    }

  caption#reference
    {
      text-align: left;
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
