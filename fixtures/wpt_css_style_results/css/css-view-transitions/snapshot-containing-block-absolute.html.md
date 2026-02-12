# css/css-view-transitions/snapshot-containing-block-absolute.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/snapshot-containing-block-absolute.html"
}
```

## style[0]

```css

:root {
  view-transition-name: none;
  background-color: red;
}

body {
  height: 400vh;
}

#target {
  position: absolute;
  left: 0px;
  top: 600px;
  width: 100px;
  height: 100px;

  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;

  background: darkseagreen;
  view-transition-name: target;
}

::view-transition-group(target) {
  animation-duration: 50s;
  animation-play-state: paused;
  top: unset;
  left: unset;
  right: 0px;
  bottom: 0px;
}

::view-transition {
  position: absolute;
  left: 20px;
  top: 40px;
  width: 700px;
  height: 500px;
  background-color: limegreen;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
