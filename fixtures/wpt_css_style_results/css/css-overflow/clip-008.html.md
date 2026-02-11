# css/css-overflow/clip-008.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/clip-008.html"
}
```

## style[0]

```css

  #testHolder {
    width: 100px;
    height: 100px;
    background-color: red;
  }
  #clipped {
    width: 100px;
    height: 100px;
    overflow-x: clip;
    background-color: wheat;
    border-radius: 50%;
  }
  #contents {
    width: 100px;
    height: 100px;
    background-color: green;
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
