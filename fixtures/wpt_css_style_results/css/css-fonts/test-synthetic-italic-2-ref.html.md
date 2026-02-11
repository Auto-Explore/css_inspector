# css/css-fonts/test-synthetic-italic-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/test-synthetic-italic-2-ref.html"
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
            #test1 {
              font-style: oblique 14deg;
              top: 100px;
              left: 100px;
            }
            #test2 {
              font-style: oblique -45deg;
              top: 100px;
              left: 200px;
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
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
