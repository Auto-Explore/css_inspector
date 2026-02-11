# css/css-scroll-snap/crashtests/fractional-covering-area-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/crashtests/fractional-covering-area-crash.html"
}
```

## style[0]

```css

  #container {
    scroll-snap-type: both mandatory;
    overflow: overlay;
    position: absolute;
    height: 33554400px;
    width: 33554400px;
  }

  #area {
    scroll-snap-align: end;
    position: absolute;
    left: 33554400px;
    top: -57971.9px;
    width: 0px;
    height: 33554400px;
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
