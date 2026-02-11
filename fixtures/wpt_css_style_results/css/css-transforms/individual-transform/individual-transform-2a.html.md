# css/css-transforms/individual-transform/individual-transform-2a.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/individual-transform/individual-transform-2a.html"
}
```

## style[0]

```css

      div {
        position: fixed;
        width: 100px;
        height: 100px;
        top: 200px;
        left: 200px;
        transform-origin: 0 0;
        border-style: solid;
        border-width: 10px 0px 10px 0px;
        border-color: lime;
        translate: 50px 50px;
        rotate: 45deg;
        scale: 2 2;
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “translate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scale”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
