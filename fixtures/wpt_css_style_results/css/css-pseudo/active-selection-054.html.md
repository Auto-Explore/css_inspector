# css/css-pseudo/active-selection-054.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-054.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      font-size: 300%;
    }

  div::selection
    {
      background-color: bar;
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
