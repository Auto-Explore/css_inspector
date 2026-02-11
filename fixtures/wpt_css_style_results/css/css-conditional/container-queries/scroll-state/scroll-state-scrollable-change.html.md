# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-change.html"
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
    --top: no;
    --bottom: no;
    @container scroll-state(scrollable: top) {
      --top: yes;
    }
    @container scroll-state(scrollable: bottom) {
      --bottom: yes;
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
