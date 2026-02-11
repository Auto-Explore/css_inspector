# css/css-tables/tentative/paint/background-image-column-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/tentative/paint/background-image-column-ref.html"
}
```

## style[0]

```css

  body {
    --peek: LightGreen;
  }
  .bg {
    background-color: var(--peek);
    background-image: linear-gradient(45deg, orange 0px, orange 10px, gainsboro 3px, gainsboro 25%, rgba(160,160,160,0.5) 25%, rgba(160,160,160,0.5) 50%, silver 50%, silver 75%, darkgray 75%, darkgray 220px, blue 220px);
    background-repeat: no-repeat;
    background-size: 50px 230px;
  }
  main * {
    box-sizing: border-box;
  }
  .td {
    width: 50px;
    height: 50px;
    position:absolute;
    border: 4px solid black;
  }
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
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
