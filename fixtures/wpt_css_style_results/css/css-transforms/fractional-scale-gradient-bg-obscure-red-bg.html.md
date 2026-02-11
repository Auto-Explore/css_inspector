# css/css-transforms/fractional-scale-gradient-bg-obscure-red-bg.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/fractional-scale-gradient-bg-obscure-red-bg.html"
}
```

## style[0]

```css

  body {
    background: red;
    margin: 0;
  }
  .container {
    width: 100vw;
    height: 100vh;
    background: white;
  }
  .transform {
    transform-origin: 0 0;
    will-change: transform;
    background: linear-gradient(white, white);
    width: 47px;
    height: 47px;
    position: absolute;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
