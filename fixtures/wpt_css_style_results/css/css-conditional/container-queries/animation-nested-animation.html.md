# css/css-conditional/container-queries/animation-nested-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/animation-nested-animation.html"
}
```

## style[0]

```css

  @keyframes outer {
    from { width: 100px; }
    to { width: 300px; }
  }
  @keyframes inner {
    from { background-color: blue; }
    to { background-color: yellow; }
  }
  #container {
    container-type: inline-size;
    animation: outer 1s linear paused;
  }
  #target {
    background-color: green;
  }

  @container (min-width: 200px) {
    #target {
      animation: inner 1s linear paused;
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
