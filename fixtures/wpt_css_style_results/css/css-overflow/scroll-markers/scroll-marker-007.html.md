# css/css-overflow/scroll-markers/scroll-marker-007.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-007.html"
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
    /* It should be impossible to disable size containment for scroll marker groups, so this declaration should have no effect. */
    display: block;
    width: fit-content;
    border: 50px solid green;
    background: red;
  }

  #scroller>*::scroll-marker {
    display: block;
    width: 200px;
    height: 200px;
    content: "";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
