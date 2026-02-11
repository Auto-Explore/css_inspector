# css/css-overflow/scroller-covered-by-empty-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroller-covered-by-empty-svg.html"
}
```

## style[0]

```css

  #scroller {
    overflow: auto;
    width: 600px;
    height: 300px;
    border: 2px solid blue;
    will-change: scroll-position;
  }

  #svg {
    position: relative;
    top: -300px;
    width: 300px;
    height: 300px;
  }

  .spacer {
    height: 200vh;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
