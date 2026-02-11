# css/css-transforms/transform-box/cssbox-stroke-box.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/cssbox-stroke-box.html"
}
```

## style[0]

```css

#target {
    width: 150px;
    height: 200px;
    margin-left: 300px;
    margin-top: 100px;
    background-color: green;
    border-left: solid 50px black;

    transform: rotate(90deg);
    transform-origin: 0 0;
    transform-box: stroke-box; /* alias for border-box */
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
