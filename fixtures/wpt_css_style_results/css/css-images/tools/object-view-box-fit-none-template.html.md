# css/css-images/tools/object-view-box-fit-none-template.html

```json
{
  "format_version": 3,
  "file": "css/css-images/tools/object-view-box-fit-none-template.html"
}
```

## style[0]

```css

.view_box_subset {
  width: 50px;
  height: 100px;
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: none;
  background-color: grey;
  margin: 5px;
}

.view_box_subset_with_position {
  width: 50px;
  height: 100px;
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: none;
  background-color: grey;
  margin: 5px;
  object-position: 10px 10px;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
