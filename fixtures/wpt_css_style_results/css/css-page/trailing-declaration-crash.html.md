# css/css-page/trailing-declaration-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-page/trailing-declaration-crash.html"
}
```

## style[0]

```css

  @page {
    @top-center {}
    size: 500px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
