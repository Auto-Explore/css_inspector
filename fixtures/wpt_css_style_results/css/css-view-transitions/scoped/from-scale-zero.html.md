# css/css-view-transitions/scoped/from-scale-zero.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/from-scale-zero.html"
}
```

## style[0]

```css

  #target {
    background-color: blue;
    height: 100px;
    width: 100px;
    position: relative;
    view-transition-name: target;
    z-index: 1;
  }
  .collapsed {
    transform: scale(0);
  }

  ::view-transition-old(target) {
    animation: -ua-view-transition-fade-out 1s -0.5s linear paused;
  }

  ::view-transition-new(target) {
    animation: -ua-view-transition-fade-in 1s -0.5s linear paused;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
