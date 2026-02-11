# css/css-lists/crashtests/chrome-legacy-propagation-remove-body-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/crashtests/chrome-legacy-propagation-remove-body-crash.html"
}
```

## style[0]

```css

  body {
    display: list-item;
  }
  div {
    /* Multicol and table display to trigger legacy layout */
    display: table-row-group;
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
