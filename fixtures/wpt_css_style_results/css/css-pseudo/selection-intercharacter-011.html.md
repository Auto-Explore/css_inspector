# css/css-pseudo/selection-intercharacter-011.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-intercharacter-011.html"
}
```

## style[0]

```css

  div
    {
      font-size: 48px;
      letter-spacing: 1em;
    }

  div::selection
    {
      background-color: yellow;
    }

  span::selection
    {
      background-color: orange;
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
