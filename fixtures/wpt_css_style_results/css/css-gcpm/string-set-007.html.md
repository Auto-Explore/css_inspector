# css/css-gcpm/string-set-007.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/string-set-007.html"
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
   string-set: title content(after);
 }

 h1::after {
 content: '-after';
 }


```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
