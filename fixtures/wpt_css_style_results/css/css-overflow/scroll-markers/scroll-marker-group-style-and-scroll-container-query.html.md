# css/css-overflow/scroll-markers/scroll-marker-group-style-and-scroll-container-query.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-style-and-scroll-container-query.html"
}
```

## style[0]

```css

  #container {
    container-type: inline-size scroll-state;
    overflow: auto;
    width: 200px;
    height: 200px;
    --container: outer;
  }
  #scroller {
    container-type: inline-size scroll-state;
    width: 400px;
    height: 400px;
    scroll-marker-group: before;
    --container: inner;
    @container not scroll-state(scrollable) {
      &::scroll-marker-group { --scroll-state-test: pass; }
    }
    @container style(--container: inner) {
      &::scroll-marker-group { --style-test: pass; }
    }
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “container-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “container-type”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
