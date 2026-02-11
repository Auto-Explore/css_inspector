# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-body-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-body-001.html"
}
```

## style[0]

```css

  html {
    /* Make sure scrollbars are created on body and not propagated to viewport */
    overflow: scroll;
  }
  body {
    height: 500px;
    overflow: scroll;
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
