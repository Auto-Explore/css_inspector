# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-layout-change-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-layout-change-002.html"
}
```

## style[0]

```css

  #scroller {
    container-type: scroll-state;
    overflow: auto;
    width: 100px;
    height: 100px;
  }
  #target {
    --scrollable: no;
    @container scroll-state(scrollable) {
      --scrollable: yes;
    }
  }
  #filler {
    height: 2000px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
