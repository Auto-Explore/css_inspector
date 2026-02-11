# css/css-overflow/clip-005.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/clip-005.html"
}
```

## style[0]

```css

  .outer {
    width: 30px;
    height: 30px;
    padding: 10px;
    margin-left: 100px;
    margin-top: 100px;
    background: black;
    outline: 2px solid grey;
  }

  .inner {
    position: relative;
    top: -20px;
    left: -40px;
    background: blue;
    height: 100px;
    width: 100px;
    opacity: 0.5;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
