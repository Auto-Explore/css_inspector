# css/css-gcpm/using-strings-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/using-strings-001.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(section);
   }
  }

 h2 {
 string-set: section content();

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
