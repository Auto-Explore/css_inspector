# css/css-position/overlay/overlay-transition-out-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition-out-rendering.html"
}
```

## style[0]

```css

  #green, #red {
    /* Force display:block both popover open or closed to not rely on
       @starting-style or display transitions. */
    display: block;
    border: none;
    width: 100vw;
    height: 100vh;
  }
  #red {
    background-color: red;
  }
  #green {
    background-color: green;
  }
  #green.transition-enabled {
    transition: overlay 60s step-end;
    transition-behavior: allow-discrete;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
