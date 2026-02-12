# css/css-anchor-position/position-visibility-no-overflow-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-no-overflow-scroll.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 150px;
    overflow: hidden;
  }

  #scroll-container {
    overflow: hidden scroll;
    scrollbar-width: none;
    width: 100%;
    height: 100%;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  .target {
    position: absolute;
    position-visibility: no-overflow;
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
