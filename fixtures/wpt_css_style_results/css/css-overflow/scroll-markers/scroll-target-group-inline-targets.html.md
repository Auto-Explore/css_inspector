# css/css-overflow/scroll-markers/scroll-target-group-inline-targets.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-target-group-inline-targets.html"
}
```

## style[0]

```css

  .stg {
    scroll-target-group: auto;

    a:target-current {
      color: green;
    }
  }

  #scroller {
    height: 100px;
    overflow: auto;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “scroll-target-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
