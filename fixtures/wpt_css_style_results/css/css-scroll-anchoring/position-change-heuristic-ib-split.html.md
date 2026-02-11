# css/css-scroll-anchoring/position-change-heuristic-ib-split.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/position-change-heuristic-ib-split.html"
}
```

## style[0]

```css

  body, h1 {
    margin: 0
  }
  .sticky {
    width: 100%;
    top: 0;
    left: 0;
  }
  .sticky > div {
    width: 100%;
    height: 50px;
    background: darkblue;
  }
  header {
    background: lightblue;
  }
  main {
    background: lightgrey;
    height: 200vh;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
