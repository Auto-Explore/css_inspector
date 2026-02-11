# css/css-contain/contain-layout-ifc-022.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-ifc-022.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      font-size: 16px;
      padding: 8px;
    }

  div#floated-left
    {
      background-color: blue;
      float: left;
      margin: 8px;
      width: 6em;
    }

  div#with-contain-layout
    {
      background-color: orange;
      contain: layout;
      width: 12em;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
