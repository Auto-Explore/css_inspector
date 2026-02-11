# css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-keyboard-scroll-on-body.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-scrolled-keyboard-scroll-on-body.html"
}
```

## style[0]

```css

html {
  overflow: scroll;
}

body {
  container-type: scroll-state;
  overflow-y: scroll;
}

body {
  height: 300px;
}

div {
  height: 1000px;
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
