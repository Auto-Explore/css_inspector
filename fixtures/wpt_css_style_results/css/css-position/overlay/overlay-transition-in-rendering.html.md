# css/css-position/overlay/overlay-transition-in-rendering.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition-in-rendering.html"
}
```

## style[0]

```css

  #transition-in, #green {
    /* Force display:block both popover open or closed to not rely on
       @starting-style or display transitions. */
    display: block;
    border: none;
    width: 100vw;
    height: 100vh;
  }
  #green {
    background-color: green;
  }
  #transition-in {
    transition: overlay 60s step-end;
    transition-behavior: allow-discrete;
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
