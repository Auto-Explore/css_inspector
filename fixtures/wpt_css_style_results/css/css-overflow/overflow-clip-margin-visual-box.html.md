# css/css-overflow/overflow-clip-margin-visual-box.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-clip-margin-visual-box.html"
}
```

## style[0]

```css

  .container {
    width: 100px;
    height: 100px;
    overflow: clip;
    padding: 10px;
    border: 10px solid black;
    margin: 10px;
    background: grey;
  }

  .inner {
    width: 150px;
    height: 150px;
    position: relative;
    top: -25px;
    left: -25px;
    background: blue;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
