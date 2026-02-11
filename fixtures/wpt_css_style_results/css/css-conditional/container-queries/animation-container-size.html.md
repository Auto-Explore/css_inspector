# css/css-conditional/container-queries/animation-container-size.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/animation-container-size.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { width: 200px; }
    to { width: 100px; }
  }
  #container {
    container-type: inline-size;
    animation: anim 1s linear paused;
  }
  #target {
    background-color: green;
  }

  @container (width: 200px) {
    #target {
      background-color: blue;
    }
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
