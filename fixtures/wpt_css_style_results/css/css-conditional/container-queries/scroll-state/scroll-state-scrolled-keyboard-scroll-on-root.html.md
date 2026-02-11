# css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-keyboard-scroll-on-root.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-keyboard-scroll-on-root.html"
}
```

## style[0]

```css

  html {
      container-type: scroll-state;
  }

  body {
      min-height: 200vh;
  }

  div {
    --y: no;
  }

  @container scroll-state(scrolled: y) {
    div {
      --y: yes;
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
