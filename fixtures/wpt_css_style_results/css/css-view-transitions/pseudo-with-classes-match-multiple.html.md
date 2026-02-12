# css/css-view-transitions/pseudo-with-classes-match-multiple.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-match-multiple.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  position: absolute;
  view-transition-class: cls some-div;
}

#target {
  background: green;
  view-transition-name: target;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation-play-state: paused;
}

::view-transition-new(target.cls.some-div),
::view-transition-old(target.cls.some-div) {
  left: 100px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
