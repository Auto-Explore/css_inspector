# css/css-values/angle-units-002.html

```json
{
  "format_version": 3,
  "file": "css/css-values/angle-units-002.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      width: 100px;
    }

  div#test-overlapping-green
    {
      background-color: red;
      background-image: linear-gradient(90DeG, green, green);
    }

  div#reference-overlapped-red
    {
      background-color: red;
      bottom: 100px;
      position: relative;
      z-index: -1;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
