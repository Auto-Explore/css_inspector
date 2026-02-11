# css/filter-effects/filter-scaling-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-scaling-001-ref.html"
}
```

## style[0]

```css

  * {
    margin: 0;
    padding: 0;
  }

  :root, #outer {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  #outer {
    transform-origin: 0 0;
    transform: scale(5);
  }

  #pad {
    /* So that scaling by five it takes half of the viewport */
    height: 10vh;
  }

  #filtered {
    width: 100%;
    height: 10vh;
    background: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
