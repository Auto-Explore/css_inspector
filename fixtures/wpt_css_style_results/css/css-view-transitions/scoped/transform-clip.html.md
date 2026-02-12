# css/css-view-transitions/scoped/transform-clip.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/transform-clip.html"
}
```

## style[0]

```css

  #clipper {
    view-transition-group: contain;
    view-transition-name: clipper;
    overflow: clip;
    height: 100px;
    width: 100px;
    border-radius: 50px;
    position: absolute;
    top: 20px;
    left: 20px;
  }

#target {
  height: 50px;
  background-color: forestgreen;
  view-transition-name: target;
  will-change: transform;
}

::view-transition-group(clipper) {
  animation-play-state: paused;
  background-color: hotpink;
  overflow: clip;
  border-radius: 50px;
}

.lower {
  transform: translateY(25px);
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
