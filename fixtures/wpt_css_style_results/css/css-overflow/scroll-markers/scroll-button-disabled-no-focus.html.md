# css/css-overflow/scroll-markers/scroll-button-disabled-no-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-button-disabled-no-focus.html"
}
```

## style[0]

```css

div::scroll-button(inline-start) {
  content: "";
  background-color: green;
}

div::scroll-button(inline-start):focus {
  background-color: red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
