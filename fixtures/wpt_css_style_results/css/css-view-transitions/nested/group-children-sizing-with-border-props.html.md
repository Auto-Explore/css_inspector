# css/css-view-transitions/nested/group-children-sizing-with-border-props.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/group-children-sizing-with-border-props.html"
}
```

## style[0]

```css

#clipper {
  view-transition-group: contain;
  view-transition-name: clipper;

  border-width: 5px 10px 15px 20px;
  border-radius: 0px 60px 50px 70px;
  corner-shape: squircle;
  border-style: solid;
  border-color: green;

  height: 200px;
  width: 200px;
}

.item {
  view-transition-name: match-element;
  background: blue;
  position: relative;
  top: -25px;
  left: -10px;

  height: 50px;
  width: 250px;
  margin: 1px;
  border: 1px solid black;
}

.item.popout {
  view-transition-group: none;
}

::view-transition-group(*),
::view-transition-old(*),
::view-transition-new(*) {
  animation-play-state: paused;
}

::view-transition-group-children(clipper) {
  overflow: clip;
}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “corner-shape”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
