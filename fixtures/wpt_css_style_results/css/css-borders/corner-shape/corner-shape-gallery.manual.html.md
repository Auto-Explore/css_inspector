# css/css-borders/corner-shape/corner-shape-gallery.manual.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-gallery.manual.html"
}
```

## style[0]

```css

section {
    display: flex;
    flex-direction: column;
    padding: 10px;
    margin: 10px;
    border: 2px solid grey;
    border-radius: 2px;
    background: lightgray;
}

label {
    overflow-y: auto;
    max-width: 250px;
    max-height: 40px;
    font-size: 10px;
}

iframe {
    overflow: clip;
    width: 500px;
    height: 300px;
    border: none;
}

main {
    display: flex;
    flex-wrap: wrap;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
