# css/css-masking/clip-path/clip-path-shape-003.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-003.html"
}
```

## style[0]

```css

  #rect {
    width: 100px;
    height: 100px;
    background-color: green;
    clip-path: shape(nonzero from 10px 10px,
                     curve to 60px 20% with 40px 0,
                     smooth to 90px 0,
                     curve by -20px 60% with 10% 40px / 20% 20px,
                     smooth by -40% -10px with -10px 70px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
