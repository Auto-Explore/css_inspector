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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
