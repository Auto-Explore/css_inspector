# css/css-gcpm/using-strings-003.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/using-strings-003.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(section, start);
   }
  }

 h2 {
 string-set: section content();
 }

 #s2 { page-break-before: always; }
#s4 { page-break-after: always; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
