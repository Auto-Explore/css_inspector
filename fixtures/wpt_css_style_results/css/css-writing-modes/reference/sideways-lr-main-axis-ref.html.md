# css/css-writing-modes/reference/sideways-lr-main-axis-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/reference/sideways-lr-main-axis-ref.html"
}
```

## style[0]

```css

    .container {
        font-size: 0
    }
    .item {
        width: 20px;
        height: 20px;
    }

    .item:nth-child(1) { background-color: lime; }
    .item:nth-child(2) { background-color: limegreen; }
    .item:nth-child(3) { background-color: green; }

    .container.reverse .item:nth-child(1) { background-color: green; }
    .container.reverse .item:nth-child(2) { background-color: limegreen; }
    .container.reverse .item:nth-child(3) { background-color: lime; }

    .container.row .item { display: inline-block; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
