# css/css-overflow/overflow-body-propagation-006.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-006.html"
}
```

## style[0]

```css

  :root {
    scroll-snap-type: both mandatory;
  }
  body {
    overflow: scroll;
    margin-top: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
