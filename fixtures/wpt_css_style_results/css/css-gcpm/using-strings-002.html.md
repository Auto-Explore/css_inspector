# css/css-gcpm/using-strings-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/using-strings-002.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(section, last);
   }
  }

 h2 {
 string-set: section content();
 }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
