# css/filter-effects/blur-clip-stacking-context-002.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/blur-clip-stacking-context-002.html"
}
```

## style[0]

```css

  #cover, #clip {
    position: absolute;
    top: 10px;
    left: 10px;
    width: 100px;
    height: 100px;
  }

  #cover {
    background: green;
  }

  #clip {
    overflow: hidden;
    background: blue;
    filter: blur(20px);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
