# css/css-animations/jump-start-animation-before-phase.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/jump-start-animation-before-phase.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  @keyframes slide {
    from { transform: translateX(100px); }
    to { transform: translateX(200px); }
  }

  #target {
    background-color: green;
    height: 100px;
    width: 100px;
    margin: 0;
    animation: slide 10000s 5000s steps(1, jump-start) backwards;
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
