# css/css-view-transitions/scoped/clip-on-target-in-callback.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/clip-on-target-in-callback.html"
}
```

## style[0]

```css

#container {
  width: 200px;
  height: 200px;
  background: blue;
  border: 5px solid black;
}
#container.after {
  overflow: clip;
}

#target {
  margin-top: 150px;
  width: 100px;
  height: 100px;
  background: green;
  view-transition-name: target;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  /* freeze all animations at start */
  animation-duration: 100000s;
  animation-timing-function: steps(1, jump-end);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
