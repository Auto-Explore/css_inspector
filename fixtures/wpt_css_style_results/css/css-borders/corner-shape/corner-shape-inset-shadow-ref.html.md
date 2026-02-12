# css/css-borders/corner-shape/corner-shape-inset-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-inset-shadow-ref.html"
}
```

## style[0]

```css

    .target {
        width: 200px;
        height: 200px;
        box-sizing: border-box;
        corner-shape: bevel notch squircle scoop;
        border-radius: 20%;
        overflow: clip;
        position: relative;
    }

    .target::before {
        content: " ";
        display: block;
        background: green;
        border: 30px solid blue;
        corner-shape: inherit;
        border-radius: inherit;
        position: absolute;
        left: -5px;
        top: 5px;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
    }

    .target::after {
        content: " ";
        display: block;
        border: 20px solid purple;
        corner-shape: inherit;
        border-radius: inherit;
        position: absolute;
        inset: 0;
        box-sizing: border-box;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
