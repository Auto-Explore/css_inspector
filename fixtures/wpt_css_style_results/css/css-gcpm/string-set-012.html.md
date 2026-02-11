# css/css-gcpm/string-set-012.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/string-set-012.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(header);
   }
  }

 h1 {
 string-set: header attr(title);
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
