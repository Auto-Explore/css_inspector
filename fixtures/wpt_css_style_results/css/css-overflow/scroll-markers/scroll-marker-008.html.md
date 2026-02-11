# css/css-overflow/scroll-markers/scroll-marker-008.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-008.html"
}
```

## style[0]

```css

  #scroller {
    overflow: hidden;
    scroll-marker-group: after;
  }

  #scroller::scroll-marker-group {
    contain: none;
    /* It should be impossible to disable layout containment for scroll marker groups, so this declaration should have no effect. */
    display: block;
    width: 100px;
    height: 100px;
    background: red;
  }

  #scroller>*::scroll-marker {
    display: block;
    position: absolute;
    right: 0;
    top: 0;
    width: 100px;
    height: 100px;
    content: "";
    background: green;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “scroll-marker-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
