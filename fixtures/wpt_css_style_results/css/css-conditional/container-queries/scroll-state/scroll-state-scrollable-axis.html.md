# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-axis.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-axis.html"
}
```

## style[0]

```css

  .scroller {
    writing-mode: vertical-lr;
    width: 200px;
    height: 200px;
    container-type: scroll-state;
    overflow: scroll;
  }
  .scroller.horizontal::after {
    content: " ";
    display: block;
    width: 10000px;
    height: 10px;
  }
  .scroller.vertical::after {
    content: " ";
    display: block;
    width: 10px;
    height: 10000px;
  }
  span {
    --inline: no;
    --block: no;
    --x: no;
    --y: no;
  }
  @container scroll-state(scrollable: inline) {
    span { --inline: yes; }
  }
  @container scroll-state(scrollable: block) {
    span { --block: yes; }
  }
  @container scroll-state(scrollable: x) {
    span { --x: yes; }
  }
  @container scroll-state(scrollable: y) {
    span { --y: yes; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
