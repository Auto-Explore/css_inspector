# css/css-overflow/overflow-body-propagation-013.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-013.html"
}
```

## style[0]

```css

:root {
  display: none;
  scrollbar-color: red red;
}
body {
  overflow: scroll;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
