# css/css-view-transitions/view-transition-types-reserved-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-types-reserved-ref.html"
}
```

## style[0]

```css


.test {
  width: 100px;
  height: 100px;
  background: green;
}
#container {
  display: grid;
  grid-template-columns: repeat(3, 100px);
  grid-gap: 10px;
}
body { background: lightpink; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
