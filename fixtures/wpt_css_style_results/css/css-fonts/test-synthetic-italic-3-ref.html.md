# css/css-fonts/test-synthetic-italic-3-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/test-synthetic-italic-3-ref.html"
}
```

## style[0]

```css

            div {
              /* use Ahem font which has no italic/oblique face, so that the oblique
                 will be synthesized with the given angle */
              font: 50px/1 Ahem;
              position: absolute;
            }
            /* test elements will use Ahem with various values of obliqueness */
            .test {
              writing-mode: vertical-rl;
              -webkit-text-orientation: upright;
              text-orientation: upright;
              color: green;
            }
            /* check that the sheared glyphs are still correct when transforms are
               also in effect */
            #test3 {
              font-style: oblique 25deg;
              top: 150px;
              left: 100px;
              transform: scale(1.5) rotate(45deg) skew(30deg);
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-orientation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
