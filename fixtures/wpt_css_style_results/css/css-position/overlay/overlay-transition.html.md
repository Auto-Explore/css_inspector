# css/css-position/overlay/overlay-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition.html"
}
```

## style[0]

```css

  #transition-in, #transition-out {
    /* Force display:block both popover open or closed to not rely on
       @starting-style or display transitions. */
    display: block;
  }
  .enable-transitions :is(#transition-in, #transition-out) {
    transition: overlay 60s step-end;
    transition-behavior: allow-discrete;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “transition-behavior”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
