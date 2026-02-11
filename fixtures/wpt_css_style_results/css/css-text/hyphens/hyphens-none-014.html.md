# css/css-text/hyphens/hyphens-none-014.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-none-014.html"
}
```

## style[0]

```css

div { font: 20px/1 Ahem; }
img { float:right; }
.test {
    max-width: 100px;
    color: green;
}
span { hyphens: none; }
.ref {
    position: absolute;
    background: green linear-gradient(red, red) 2ch 0/3ch 3ch no-repeat;
    color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
}
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
