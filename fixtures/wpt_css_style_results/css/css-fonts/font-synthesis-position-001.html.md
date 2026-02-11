# css/css-fonts/font-synthesis-position-001.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-synthesis-position-001.html"
}
```

## style[0]

```css

    /* Lato has superscript Latin letters, but not subscript ones;
     * digits are available in both super- and subscript form.
     */
    @font-face {
        font-family: "lato";
        src: url(/fonts/Lato-Medium.ttf);
    }
    @supports not (font-synthesis-position: none) {
        .test::before {
            color: red;
            content: "font-synthesis-position is unsupported!"
        }
    }
    .test {
        font-family: "lato";
        font-size: 2em;
    }
    .nosynth {
        font-synthesis-position: none;
    }
    .sub {
        font-variant-position: sub;
    }
    .super {
        font-variant-position: super;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “font-synthesis-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
