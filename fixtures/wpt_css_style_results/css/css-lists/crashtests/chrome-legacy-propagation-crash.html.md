# css/css-lists/crashtests/chrome-legacy-propagation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/crashtests/chrome-legacy-propagation-crash.html"
}
```

## style[0]

```css

  html {
    display: list-item;
    direction: rtl;
  }
  body {
    direction: ltr;
  }
  div {
    display:table-column-group;
    column-count: 1;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
