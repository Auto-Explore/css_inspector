# css/css-conditional/container-queries/animation-nested-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/animation-nested-transition.html"
}
```

## style[0]

```css

  @keyframes outer {
    from { width: 100px; }
    to { width: 300px; }
  }
  #container {
    container-type: inline-size;
    animation: outer 1s linear paused;
  }
  #target {
    background-color: rgb(100, 100, 100);
  }

  @container (min-width: 200px) {
    #target {
      transition: background-color 100s steps(2, start);
      background-color: rgb(200, 200, 200);
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
