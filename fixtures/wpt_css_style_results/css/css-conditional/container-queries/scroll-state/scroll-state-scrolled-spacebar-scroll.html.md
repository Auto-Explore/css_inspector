# css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-spacebar-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-spacebar-scroll.html"
}
```

## style[0]

```css

  #scroller {
    width: 200px;
    height: 200px;
    container-type: scroll-state;
    overflow-y: scroll;
  }
  #filler {
    height: 600px;
  }
  #target {
    --none: no;
    --y: no;
    @container scroll-state(scrolled: none) {
      --none: yes;
    }
    @container scroll-state(scrolled: y) {
      --y: yes;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
