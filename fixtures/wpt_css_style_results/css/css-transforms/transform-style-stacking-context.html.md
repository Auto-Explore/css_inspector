# css/css-transforms/transform-style-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-style-stacking-context.html"
}
```

## style[0]

```css

  div {
    width: 100px;
    height: 100px;
  }
  #front {
    background-color: lime;
    /* makes a stacking context and puts this on top */
    position: absolute;
    z-index: 10;
  }
  #back {
    transform-style: preserve-3d;
  }
  #notOnTop {
    background-color: red;
    /* z-index is higher than on #front, but this should still be covered up because it is inside #back, which has 'transform-style: preserve-3d' */
    position: absolute;
    z-index: 1000;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
