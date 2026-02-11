# css/css-pseudo/active-selection-004-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-004-manual.html"
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
      text-decoration: fuchsia underline dotted;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
