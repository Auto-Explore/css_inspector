# css/css-contain/contain-paint-stacking-context-001b.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-stacking-context-001b.html"
}
```

## style[0]

```css

  div {
    width: 100px;
    height: 100px;
  }
  #front {
    background-color: green;
    /* makes a stacking context and puts this on top */
    position: absolute;
    z-index: 10;
  }
  #back {
    will-change: contain;
  }
  #notOnTop {
    background-color: red;
    /* z-index is higher than on #front, but this should still be covered up because it is inside #back, which has 'will-change: contain' */
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
