# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-root.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-root.html"
}
```

## style[0]

```css

  :root {
    container-type: scroll-state;
  }
  #target {
    height: 5000px;
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
