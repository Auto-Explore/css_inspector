# css/css-view-transitions/new-content-has-scrollbars-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-has-scrollbars-ref.html"
}
```

## style[0]

```css

  html, body {
    width: 100%;
    height: 100%;
    background-color: lightpink;
  }
  body {
    margin: 50px;
  }
  div {
    background-image:
      linear-gradient(45deg, #000 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #000 75%),
      linear-gradient(45deg, transparent 75%, #000 75%),
      linear-gradient(45deg, #000 25%, lightgreen 25%);
    background-size: 200px 200px;
    background-position: 0 0, 0 0, -100px -100px, 100px 100px;
    width: 200%;
    height: 200%;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
