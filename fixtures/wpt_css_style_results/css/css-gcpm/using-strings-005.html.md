# css/css-gcpm/using-strings-005.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/using-strings-005.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(title, last);
   }
  }

 h2 {
 string-set: title content();
 }

#s2, #s4 { page-break-before: always; }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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
