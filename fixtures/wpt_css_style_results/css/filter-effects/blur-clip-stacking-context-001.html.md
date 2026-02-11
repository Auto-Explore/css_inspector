# css/filter-effects/blur-clip-stacking-context-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/blur-clip-stacking-context-001.html"
}
```

## style[0]

```css

  #cover, #clip, #blur {
    width: 100px;
    height: 100px;
  }

  #cover, #clip {
    position: absolute;
    top: 10px;
    left: 10px;
  }

  #cover {
    background: green;
  }

  #clip {
    overflow: hidden;
    background: red;
  }

  #blur {
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
