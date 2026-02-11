# css/css-pseudo/active-selection-053.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-053.html"
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
      color: foo;
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
