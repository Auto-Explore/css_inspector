# css/css-pseudo/highlight-cascade/cascade-highlight-005.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/cascade-highlight-005.html"
}
```

## style[0]

```css

  div
    {
      font-size: 300%;
    }

  div::selection
    {
      background-color: green;
      color: yellow;
    }

  span::selection
    {
      color: orange;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
