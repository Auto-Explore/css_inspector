# css/css-syntax/trailing-braces.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/trailing-braces.html"
}
```

## style[0]

```css

  #target1 {
    color:green;
    color:red{};
    color:red {};
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
