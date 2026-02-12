# css/css-position/overlay/overlay-transition-finished.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition-finished.html"
}
```

## style[0]

```css

  #transition-in, #red {
    /* Force display:block both popover open or closed to not rely on
       @starting-style or display transitions. */
    display: block;
    border: none;
    width: 100vw;
    height: 100vh;
  }
  #red {
    position: fixed;
    background-color: red;
  }
  #transition-in {
    transition: overlay 0.1s step-end;
    transition-behavior: allow-discrete;
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
