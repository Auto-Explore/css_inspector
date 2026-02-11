# css/css-position/position-absolute-crash-chrome-005.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-crash-chrome-005.html"
}
```

## style[0]

```css

  .container {
    position: relative;
  }
  .abs {
    position: absolute;
    width: 50px;
    height: 50px;
    background: green;
  }
  @keyframes slidein {
    from { transform: scaleX(0); }
    to   { transform: scaleX(1); }
  }
  .animate {
    animation: slidein 0.1s linear;
  }
  .boundary {
    overflow: hidden;
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
