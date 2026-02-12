# css/css-gcpm/using-strings-004.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/using-strings-004.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(title, first-except);
   }
  }

 h1 {
 string-set: title content();
 }

#s2, #s3, #s4 { page-break-before: always; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
