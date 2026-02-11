# css/css-lists/deep-pseudo-element-remove-update.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/deep-pseudo-element-remove-update.html"
}
```

## style[0]

```css

  .container {
    counter-reset: plm-table 0;
  }

  .numbered span::after {
    content: counter(plm-table);
    counter-increment: plm-table;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
