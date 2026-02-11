# css/css-conditional/container-queries/animation-container-type-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/animation-container-type-dynamic.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { width: 200px; }
    to { width: 300px; }
  }
  #container {
    container-type: inline-size;
    animation: anim 1s linear paused;
  }
  #target {
    background-color: red;
  }

  #intermediate {
    width: 100px;
  }

  @container (min-width: 250px) {
    #intermediate {
      container-type: inline-size;
    }
  }

  @container (width: 200px) {
    #target {
      background-color: blue;
    }
  }

  @container (width: 100px) {
    /* Initially queries  #container, but later queries #intermediate, when
       the other container query starts matching. */
    #target {
      background-color: green;
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
