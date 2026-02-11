# css/css-tables/tentative/paint/background-image-column-collapsed.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/tentative/paint/background-image-column-collapsed.html"
}
```

## style[0]

```css

  body {
    --peek: LightGreen;
  }
  .bg {
    background-color: var(--peek);
    background-image: linear-gradient(90deg, orange 0px, orange 10px, gainsboro 3px, gainsboro 25%, rgba(160,160,160,0.5) 25%, rgba(160,160,160,0.5) 50%, silver 50%, silver 75%, darkgray 75%, darkgray 170px, blue 170px);
    background-repeat: no-repeat;
  }
  main * {
    box-sizing: border-box;
  }
  main table {
    border-spacing: 10px;
    border: 10px solid transparent;
    padding: 10px;
    border-collapse: collapse;
  }
  main table:hover {
    border: 10px solid yellow;
  }
  main td {
    width: 50px;
    height: 50px;
    padding: 0px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
