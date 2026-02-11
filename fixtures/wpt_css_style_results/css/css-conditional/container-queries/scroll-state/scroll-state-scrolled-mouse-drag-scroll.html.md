# css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-mouse-drag-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-mouse-drag-scroll.html"
}
```

## style[0]

```css

  #scroller {
    width: 200px;
    height: 200px;
    container-type: scroll-state;
    overflow: scroll;
  }
  #filler {
    height: 600px;
    width: 600px;
  }
  #target {
    --none: no;
    --x: no;
    --y: no;
    @container scroll-state(scrolled: none) {
      --none: yes;
    }
    @container scroll-state(scrolled: y) {
      --y: yes;
    }
    @container scroll-state(scrolled: x) {
      --x: yes;
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
