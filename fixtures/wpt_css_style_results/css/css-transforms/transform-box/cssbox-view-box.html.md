# css/css-transforms/transform-box/cssbox-view-box.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/cssbox-view-box.html"
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
    transform-box: view-box; /* acts like border-box on css boxes */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
