# css/css-color/light-dark-currentcolor-in-color.html

```json
{
  "format_version": 3,
  "file": "css/css-color/light-dark-currentcolor-in-color.html"
}
```

## style[0]

```css

  #parent { color: green; }
  #child {
    color-scheme: dark;
    color: light-dark(red, currentColor);
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
