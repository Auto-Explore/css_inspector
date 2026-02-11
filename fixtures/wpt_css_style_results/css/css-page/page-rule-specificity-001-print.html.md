# css/css-page/page-rule-specificity-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-rule-specificity-001-print.html"
}
```

## style[0]

```css


/* WPT Print Reftest default size is 5x3in - this should only change that for the first page */
@page :first {
  size: portrait;
}
div:first-of-type {
  break-after: page;
}
body {
  margin: 0;
}

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
