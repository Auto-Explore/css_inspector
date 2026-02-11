# css/css-view-transitions/snapshot-containing-block-absolute-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/snapshot-containing-block-absolute-ref.html"
}
```

## style[0]

```css

:root {
  background-color: red;
}

body {
  height: 400vh;
}

#target {
  position: absolute;
  bottom: 0px;
  right: 0px;
  width: 100px;
  height: 100px;

  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;

  background: darkseagreen;
}

#view-transition {
  position: absolute;
  left: 20px;
  top: 640px;
  width: 700px;
  height: 500px;
  background-color: limegreen;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
