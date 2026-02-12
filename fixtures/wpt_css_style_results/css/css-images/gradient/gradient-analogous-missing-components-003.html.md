# css/css-images/gradient/gradient-analogous-missing-components-003.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-analogous-missing-components-003.html"
}
```

## style[0]

```css

    .test {
        margin: 10px 50px;
        width: 200px;
        height: 50px;
        border: 1px solid black;
        --stop2: rgb(0 255 0); /* lime  */
    }

    .test1 {
        background: linear-gradient(to right in hsl shorter hue, color(srgb none 1 none), var(--stop2));
    }

    .test2 {
        background: linear-gradient(to right in hsl increasing hue, color(srgb none 1 none), var(--stop2));
    }

    .test3 {
        background: linear-gradient(to right in hsl decreasing hue, color(srgb none 1 none), var(--stop2));
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
