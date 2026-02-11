# css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrollable-pseudo.html"
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
  #wrapper {
    width: 10px;
    height: 10px;
  }
  #wrapper.large {
    width: 75px;
    height: 75px;
  }
  #target {
    width: 200%;
    height: 200%;
    --before: no;
    --after: no;
    @container scroll-state(scrollable) {
      &::before {
        --before: yes;
        content: " ";
      }
      &::after {
        --after: yes;
      }
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
