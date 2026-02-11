# css/css-view-transitions/root-style-change-during-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/root-style-change-during-animation.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  contain: paint;
  background: blue;
  view-transition-name: target;
}

#hidden {
  background: pink;
  width: 10px;
  height: 10px;
  view-transition-name: hidden;
  contain: paint;
}

html::view-transition {
  background: grey;
}

html::view-transition-group(hidden) {
  animation-duration: 500s;
  visibility: hidden;
}

html::view-transition-group(root) {
  visibility: hidden;
}

.test::view-transition-group(target) {
  background: green;
}
.test::view-transition-image-pair(target) {
  visibility: hidden;
}
```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
