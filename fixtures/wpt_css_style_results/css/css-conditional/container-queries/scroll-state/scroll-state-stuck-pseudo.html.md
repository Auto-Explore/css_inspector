# css/css-conditional/container-queries/scroll-state/scroll-state-stuck-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-stuck-pseudo.html"
}
```

## style[0]

```css

  #scroller {
    overflow-y: scroll;
    height: 200px;
  }
  #filler {
    height: 100px;
  }
  #stuck {
    #inner {
      height: 50px;
    }
    container-type: scroll-state;
    position: sticky;
    bottom: 0;
    --before: no;
    --after: no;
    @container scroll-state(stuck: bottom) {
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
