# css/css-contain/contain-content-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-content-004.html"
}
```

## style[0]

```css

  table
    {
      background-color: blue;
      border-spacing: 2px;
      height: 206px;
      table-layout: fixed;
      width: 206px;
    }

  td
    {
      background-color: white;
      padding: 0px;
      vertical-align: top;
    }

  td#contain
    {
      contain: content;
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
