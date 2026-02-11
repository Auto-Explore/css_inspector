# css/css-transforms/transform-box/cssbox-initial.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/cssbox-initial.html"
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
