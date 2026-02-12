# css/css-view-transitions/scoped/display-inline-block.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/display-inline-block.html"
}
```

## style[0]

```css

  #target {
    display: inline-block;
    view-transition-name: target;
  }

  ::view-transition-group(target),
  ::view-transition-old(target) {
    animation: unset;
  }

  @keyframes colorize {
    to { opacity: 0.5; }
  }

  ::view-transition-new(target) {
   animation: colorize 1s paused steps(1, jump-start);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
