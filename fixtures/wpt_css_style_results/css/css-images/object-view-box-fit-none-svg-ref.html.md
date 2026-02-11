# css/css-images/object-view-box-fit-none-svg-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/object-view-box-fit-none-svg-ref.html"
}
```

## style[0]

```css

div {
  margin: 5px;
}

video {
  object-fit: fill;
}

.container_view_box_subset {
  width: 50px;
  height: 100px;
  overflow: hidden;
  display: inline-block;
  background-color: grey;
}
.view_box_subset {
  position: relative;
  top: -25px;
}

.container_view_box_subset_with_position {
  width: 50px;
  height: 100px;
  overflow: hidden;
  display: inline-block;
  background-color: grey;
}
.view_box_subset_with_position {
  position: relative;
  top: -40px;
  left: 10px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
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
