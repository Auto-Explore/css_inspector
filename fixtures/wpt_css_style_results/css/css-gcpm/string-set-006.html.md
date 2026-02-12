# css/css-gcpm/string-set-006.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/string-set-006.html"
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
   string-set: title content(before);
 }

 h1::before {
 content: 'before-';
 }


```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
