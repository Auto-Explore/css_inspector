# css/css-gcpm/string-set-003.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/string-set-003.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(title);
   }
  }

 h1 {
   string-set: title content(text);
 }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “string-set”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
