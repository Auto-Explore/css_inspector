# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-container-type-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-container-type-change.html"
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
    width: 200px;
    height: 200px;
    --scrollable: no;
    @container scroll-state(scrollable) {
      --scrollable: yes;
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
