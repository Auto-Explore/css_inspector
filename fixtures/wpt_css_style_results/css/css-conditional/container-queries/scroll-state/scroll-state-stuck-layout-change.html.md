# css/css-conditional/container-queries/scroll-state/scroll-state-stuck-layout-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-stuck-layout-change.html"
}
```

## style[0]

```css

  #scroller {
    overflow-y: scroll;
    height: 300px;
  }
  #filler {
    height: 100px;
  }
  #stuck {
    container-type: scroll-state;
    position: sticky;
    bottom: 0;
    height: 100px;
    background-color: teal;
  }
  #target {
    --stuck: no;
    @container scroll-state(stuck: bottom) {
      --stuck: yes;
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
