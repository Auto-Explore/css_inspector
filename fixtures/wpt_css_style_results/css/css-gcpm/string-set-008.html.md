# css/css-gcpm/string-set-008.html

```json
{
  "format_version": 3,
  "file": "css/css-gcpm/string-set-008.html"
}
```

## style[0]

```css

  @page {
   @top-center {
   content: string(center);
   }
   @top-left {
   content: string(left);
   }
   @top-right {content:
   string(right);
   }
  }

 h1 {
   string-set: left content(), right content(), center content(first-letter);
 }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
