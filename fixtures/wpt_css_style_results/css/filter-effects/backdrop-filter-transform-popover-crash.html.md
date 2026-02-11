# css/filter-effects/backdrop-filter-transform-popover-crash.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-transform-popover-crash.html"
}
```

## style[0]

```css

* {
  min-width: 1952643579% !important;
  transform: skew(0, 1.8turn) scale(1.5794431017420837e+38) translate(215mm, 62%) scaleY(55498.5);
  border-top-left-radius: 98779902.05ch;
  backdrop-filter: sepia() invert(870236770%) drop-shadow(16Q -79.35pc 125vw hsla(-1.93deg, 66%, 84%));
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “backdrop-filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
