# css/css-conditional/container-queries/scroll-state/scroll-state-initially-scrollable.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-initially-scrollable.html"
}
```

## style[0]

```css

  .scroller {
    width: 200px;
    height: 200px;
    container-type: scroll-state;
  }
  .auto {
    overflow-y: auto;
  }
  .scroll {
    overflow-y: scroll;
  }
  .clip {
    overflow-y: clip;
  }
  .scrollable::after {
    content: " ";
    display: block;
    height: 10000px;
  }
  span {
    --top: no;
    --bottom: no;
    --none: no;
  }
  @container scroll-state(scrollable: top) {
    span { --top: yes; }
  }
  @container scroll-state(scrollable: bottom) {
    span { --bottom: yes; }
  }
  @container scroll-state(scrollable: none) {
    span { --none: yes; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
