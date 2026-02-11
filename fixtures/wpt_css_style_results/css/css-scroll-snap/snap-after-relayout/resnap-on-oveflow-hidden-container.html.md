# css/css-scroll-snap/snap-after-relayout/resnap-on-oveflow-hidden-container.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-relayout/resnap-on-oveflow-hidden-container.html"
}
```

## style[0]

```css

#scroller {
  display: flex;
  overflow-x: hidden;
  scroll-snap-align: start;
  scroll-snap-type: x mandatory;
  width: 500px;
  height: 200px;
  position: absolute;
}
.child {
  display: flex;
  flex: 0 0 500px;
  scroll-snap-align: start;
  width: 500px;
  height: 100%;
  align-items: center;
  justify-content: center;
  font-size: 30px;
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
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
