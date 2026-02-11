# css/css-scroll-snap/snap-at-user-scroll-end.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-at-user-scroll-end.html"
}
```

## style[0]

```css

html {
  margin: 0px;
  scroll-snap-type: both mandatory;
}
#content {
  width: 2000px;
  height: 2000px;
  padding: 0px;
  margin: 0px;
}
#target {
  position: relative;
  left: 400px;
  top: 400px;
  width: 400px;
  height: 400px;
  background-color: lightblue;
  overflow: hidden;
  scroll-snap-align: start;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
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
