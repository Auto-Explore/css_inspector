# css/css-viewport/zoom/reference/filters-drop-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/reference/filters-drop-shadow-ref.html"
}
```

## style[0]

```css

body {
    --scale: 1;
}
.container {
    filter: drop-shadow(calc(5px * var(--scale)) calc(5px * var(--scale)) calc(5px * var(--scale)) black);
    padding: calc(20px * var(--scale));
}
.circle-mask {
    border-radius: calc(80px * var(--scale));;
    overflow: hidden;
    width: calc(100px * var(--scale));
    height: calc(100px * var(--scale));
}
.green-box {
    width: calc(100px * var(--scale));
    height: calc(100px * var(--scale));
    background-color: green;
    font-size: calc(1rem * var(--scale));
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
}
.zoom {
    --scale: 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
