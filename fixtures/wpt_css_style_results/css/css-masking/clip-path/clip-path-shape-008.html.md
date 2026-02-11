# css/css-masking/clip-path/clip-path-shape-008.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-008.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  #shape {
    width: 240px;
    height: 240px;
    padding: 10px;
    border: 10px solid red;
    box-sizing: border-box;
    background-color: green;
    /* The size of the content-box is 200x200. */
    clip-path: shape(from center left,
                    curve by 200px 0 with 50% -50% from start / -50% -50% from end,
                    curve by -200px 0 with 50% 50% from end / -50% 50% from start,
                    close)
               content-box;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
