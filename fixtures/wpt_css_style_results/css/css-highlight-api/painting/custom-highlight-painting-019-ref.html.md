# css/css-highlight-api/painting/custom-highlight-painting-019-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-019-ref.html"
}
```

## style[0]

```css

  body {
    text-decoration: 2px green underline;
  }
  .foo {
    color:blue;
    text-decoration: 2px blue underline;
  }
  .bar {
    text-decoration: 2px currentColor underline;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
