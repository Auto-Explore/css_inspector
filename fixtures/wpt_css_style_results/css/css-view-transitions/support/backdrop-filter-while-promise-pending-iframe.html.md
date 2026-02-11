# css/css-view-transitions/support/backdrop-filter-while-promise-pending-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/support/backdrop-filter-while-promise-pending-iframe.html"
}
```

## style[0]

```css

body {
  background: green;
  margin: 0;
}

.backdrop-filter {
  width: 100px;
  height: 100px;
  border: 1px solid white;
  backdrop-filter: invert(1);
  view-transition-name: foo;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
