# css/css-masking/clip-path/animations/clip-path-animation-cancel.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/animations/clip-path-animation-cancel.html"
}
```

## style[0]

```css

  /* This test ensures that canceling an animation properly results in a
     repaint. If this does not happen, the animation will remain stuck until
     invalidated for another reason. */
  .container {
    width: 100px;
    height: 100px;
    background-color: green;
    animation: clippath 10s;
    clip-path: circle(20% at 20% 20%);
  }

  @keyframes clippath {
    0% {
      clip-path: circle(35% at 35% 35%);
    }

    100% {
      clip-path: circle(50% at 50% 50%);
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
