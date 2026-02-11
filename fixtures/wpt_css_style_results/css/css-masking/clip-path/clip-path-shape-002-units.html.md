# css/css-masking/clip-path/clip-path-shape-002-units.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-002-units.html"
}
```

## style[0]

```css

  * {
    font-size: 16px;
  }

  html {
    font-size: 10px;
    --v50: 50px;
  }

  #rect {
    width: 100px;
    height: 100px;
    font-size: 5px;
    font-family: Ahem;
    background-color: green;
    clip-path: shape(evenodd from 10px 2ch,
                     hline by 80px, vline by 80%, hline by -8rem, close,
                     move to 25% 25px, hline by 10em, vline by var(--v50), hline by -50%);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
